﻿$ErrorActionPreference='Stop'
$installDir=[System.IO.Path]::GetFullPath((Join-Path $env:LOCALAPPDATA 'Programs\Island'))
if([System.IO.Path]::GetFullPath($PSScriptRoot) -ne $installDir){throw '卸载脚本只能在Island安装目录中运行。'}
$exePath=Join-Path $installDir 'message-island.exe'
Get-Process message-island -ErrorAction SilentlyContinue | Where-Object {$_.Path -eq $exePath} | ForEach-Object {
    $null=$_.CloseMainWindow();if(-not $_.WaitForExit(3000)){Stop-Process -Id $_.Id}
}
$shell=New-Object -ComObject WScript.Shell
$links=@((Join-Path $shell.SpecialFolders.Item('Desktop') 'Island.lnk'),(Join-Path $env:APPDATA 'Microsoft\Windows\Start Menu\Programs\Island.lnk'),(Join-Path $env:APPDATA 'Microsoft\Windows\Start Menu\Programs\Island设置.lnk'),(Join-Path $env:APPDATA 'Microsoft\Windows\Start Menu\Programs\收集Island诊断.lnk'))
foreach($path in $links){if((Test-Path -LiteralPath $path) -and $shell.CreateShortcut($path).TargetPath -eq $exePath){Remove-Item -LiteralPath $path}}
$runKey='HKCU:\Software\Microsoft\Windows\CurrentVersion\Run'
Remove-ItemProperty -Path $runKey -Name Island -ErrorAction SilentlyContinue
Remove-Item -LiteralPath 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\Island' -ErrorAction SilentlyContinue
foreach($name in @('message-island.exe','message-island-previous.exe','THIRD_PARTY_NOTICES.txt','LICENSE','uninstall.ps1','收集 Island 诊断.cmd','scripts\collect-diagnostics.ps1')){
    $path=Join-Path $installDir $name;if(Test-Path -LiteralPath $path){Remove-Item -LiteralPath $path}
}
$tools=Join-Path $installDir 'scripts'
if((Test-Path -LiteralPath $tools) -and -not(Get-ChildItem -LiteralPath $tools -Force)){Remove-Item -LiteralPath $tools}
if(-not(Get-ChildItem -LiteralPath $installDir -Force)){Remove-Item -LiteralPath $installDir}
Write-Output 'Island已卸载。待办、饮水记录和设置仍保存在本机用户数据目录中。'
