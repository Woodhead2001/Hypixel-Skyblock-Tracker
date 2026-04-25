@echo off
set OUTPUT=project_dump.txt

echo Creating project dump...
echo. > %OUTPUT%

echo --- PROJECT DUMP START --- >> %OUTPUT%
echo. >> %OUTPUT%

for /r %%F in (*.rs *.toml *.json *.js *.ts *.jsx *.tsx *.css *.html) do (
    echo --- FILE: %%F --- >> %OUTPUT%
    type "%%F" >> %OUTPUT%
    echo. >> %OUTPUT%
)

echo --- PROJECT DUMP END --- >> %OUTPUT%

echo Done!
echo Your dump is in %OUTPUT%
pause
