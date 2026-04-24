#Requires -RunAsAdministrator

<#
.SYNOPSIS
    安装 ShutdownRemote 应用并配置开机自启。
.DESCRIPTION
    1. 将 ./dist 目录复制到用户主目录下的 shutdown-remote 文件夹。
    2. 创建计划任务，在系统启动时运行 shutdown-auto.exe。
#>

# 定义变量
$SourceDir = Join-Path $PSScriptRoot "dist"
$TargetDir = Join-Path $env:USERPROFILE "shutdown-remote"
$ExeName = "shutdown-auto.exe"
$TaskName = "ShutdownRemoteAutoStart"
$TaskDescription = "Automatically start ShutdownRemote agent on system startup."

# 检查源目录是否存在
if (-not (Test-Path $SourceDir)) {
    Write-Error "错误: 找不到源目录 '$SourceDir'。请确保在包含 dist 文件夹的目录下运行此脚本。"
    exit 1
}

# 检查关键 exe 是否存在
$ExePathInDist = Join-Path $SourceDir $ExeName
if (-not (Test-Path $ExePathInDist)) {
    Write-Warning "警告: 在 dist 目录中未找到 '$ExeName'。计划任务可能无法正常工作。"
}

Write-Host "正在安装 ShutdownRemote..." -ForegroundColor Cyan

# 1. 复制文件到用户目录
try {
    # 如果目标目录已存在，先删除以保证更新干净
    if (Test-Path $TargetDir) {
        Write-Host "检测到旧版本，正在清理..." -ForegroundColor Yellow
        Remove-Item -Path $TargetDir -Recurse -Force
    }

    Write-Host "正在复制文件到 $TargetDir ..." -ForegroundColor Green
    Copy-Item -Path $SourceDir -Destination $TargetDir -Recurse -Force
    
    Write-Host "文件复制完成。" -ForegroundColor Green
}
catch {
    Write-Error "文件复制失败: $_"
    exit 1
}

# 2. 创建计划任务
$FinalExePath = Join-Path $TargetDir $ExeName

try {
    # 检查是否已存在同名任务，如果存在则删除
    $ExistingTask = Get-ScheduledTask -TaskName $TaskName -ErrorAction SilentlyContinue
    if ($ExistingTask) {
        Write-Host "发现已存在的计划任务，正在卸载..." -ForegroundColor Yellow
        Unregister-ScheduledTask -TaskName $TaskName -Confirm:$false
    }

    # 定义触发器：系统启动时
    $Trigger = New-ScheduledTaskTrigger -AtStartup
    
    # 定义动作：运行 exe
    $Action = New-ScheduledTaskAction -Execute $FinalExePath
    
    # 定义设置：允许硬终止，隐藏窗口等可选配置
    $Settings = New-ScheduledTaskSettingsSet -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries -StartWhenAvailable
    
    # 注册任务 (以 SYSTEM 账户运行，确保最高权限和后台运行，或者使用 CURRENT_USER)
    # 这里使用 SYSTEM 账户通常更适合后台服务类应用，且不需要用户登录即可运行
    # 如果需要仅在用户登录后运行，可将 -User 改为 $env:USERNAME 并去掉 -RunLevel Highest
    Register-ScheduledTask -TaskName $TaskName `
                           -Trigger $Trigger `
                           -Action $Action `
                           -Settings $Settings `
                           -User "SYSTEM" `
                           -RunLevel Highest `
                           -Description $TaskDescription `
                           -Force | Out-Null

    Write-Host "计划任务 '$TaskName' 创建成功。" -ForegroundColor Green
    Write-Host "安装完成！" -ForegroundColor Cyan
    Write-Host "应用路径: $TargetDir"
    Write-Host "下次重启电脑后，$ExeName 将自动启动。"

}
catch {
    Write-Error "创建计划任务失败: $_"
    exit 1
}