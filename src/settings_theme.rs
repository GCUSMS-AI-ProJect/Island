//! 设置窗口独立外观，不改变岛体的绘制样式。
use eframe::egui::{self,Color32};

#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub enum SettingsTheme { #[default] System, Light, Dark }
impl SettingsTheme {
    pub fn key(self)->&'static str {match self {Self::System=>"system",Self::Light=>"light",Self::Dark=>"dark"}}
    pub fn parse(value:&str)->Self {match value {"light"=>Self::Light,"dark"=>Self::Dark,_=>Self::System}}
    pub fn dark(self,system_dark:bool)->bool {match self {Self::System=>system_dark,Self::Light=>false,Self::Dark=>true}}
}

pub fn system_dark(ctx:&egui::Context)->bool {
    let id=egui::Id::new("settings-system-theme");
    if let Some((at,dark))=ctx.data(|d|d.get_temp::<(std::time::Instant,bool)>(id)) {
        if at.elapsed()<std::time::Duration::from_secs(1){return dark;}
    }
    #[cfg(windows)]
    let dark={
        use windows::{core::w,Win32::System::Registry::*};
        let mut value=1u32;let mut size=4u32;
        unsafe { let _=RegGetValueW(HKEY_CURRENT_USER,w!("Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize"),
            w!("AppsUseLightTheme"),RRF_RT_REG_DWORD,None,Some((&mut value as *mut u32).cast()),Some(&mut size)); }
        value==0
    };
    #[cfg(not(windows))]
    let dark=ctx.input(|i|i.raw.system_theme)==Some(egui::Theme::Dark);
    ctx.data_mut(|d|d.insert_temp(id,(std::time::Instant::now(),dark)));
    dark
}

pub fn color(ctx:&egui::Context,dark:Color32,light:Color32)->Color32 {
    if ctx.style().visuals.dark_mode {dark}else{light}
}

/// 品牌强调色（靛蓝紫）：开关、侧栏选中、主按钮、滑块共用一种颜色。
pub fn accent(ctx:&egui::Context)->Color32 {color(ctx,Color32::from_rgb(130,124,255),Color32::from_rgb(92,84,236))}
/// 强调色的浅底：侧栏选中块、选中文字底色。
pub fn accent_soft(ctx:&egui::Context)->Color32 {color(ctx,Color32::from_rgb(52,49,92),Color32::from_rgb(226,224,253))}
/// 强调色上的文字颜色。
pub const ON_ACCENT:Color32=Color32::WHITE;

/// 两色线性混合，t=0 为 a，t=1 为 b。
pub fn mix(a:Color32,b:Color32,t:f32)->Color32 {
    let t=t.clamp(0.0,1.0);
    let l=|x:u8,y:u8|(x as f32+(y as f32-x as f32)*t).round() as u8;
    Color32::from_rgba_premultiplied(l(a.r(),b.r()),l(a.g(),b.g()),l(a.b(),b.b()),l(a.a(),b.a()))
}

/// 设置页的弹簧动效：与岛体共用同一套带速度弹簧，目标随时改变也不会跳变。
/// 首次出现时直接落在目标上；未静止时自动请求重绘。
pub fn spring(ctx:&egui::Context,id:egui::Id,target:f32,stiffness:f32,zeta:f32)->f32 {
    let dt=ctx.input(|i|i.stable_dt).min(0.05);
    let mut s=ctx.data(|d|d.get_temp::<crate::anim::Spring>(id)).unwrap_or_else(||crate::anim::Spring::new(target,stiffness,zeta));
    s.target=target;
    s.step(dt);
    let scale=target.abs().max(1.0);
    if s.settled(scale) {s.snap(target);} else {ctx.request_repaint();}
    ctx.data_mut(|d|d.insert_temp(id,s));
    s.pos
}
