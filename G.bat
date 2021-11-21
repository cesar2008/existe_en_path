@echo off
if '%1'=='' goto ERROR
git init
git add .
git remote add origin git@github.com:cesar2008/existe_en_path.git
git commit -m %1
git push -u origin master
goto FIN

:ERROR
@echo    FALTA MENSAJE PARA EL COMMIT (ENTRE COMILLAS)

:FIN