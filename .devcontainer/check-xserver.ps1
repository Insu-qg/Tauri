$vcxsrv = Get-Process vcxsrv -ErrorAction SilentlyContinue

if ($null -eq $vcxsrv) {
    Write-Host "VcXsrv n'est pas en cours d'exécution. Démarrage de VcXsrv..."
    Start-Process "C:\Program Files\VcXsrv\vcxsrv.exe" -ArgumentList ":0 -multiwindow -ac"
    Start-Sleep -Seconds 2
}
else {
    Write-Host "VcXsrv est déjà en cours d'exécution."
}