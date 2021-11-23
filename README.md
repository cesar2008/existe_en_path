# existe_en_path
##
## version 0.1.0
##
## _Chequea si existe una ruta determinada en la variable de entorno PATH_


**existe_en_path** es una peque¤a aplicacion de consola que comprueba si existe una determinada ruta en la variable de entorno PATH de Windows.



## Caracteristicas

- Rapido
- Simple y sencillo
- Devuelve c¢digo de salida 
- No necesita instalaci¢n



## Tecnologia
**existe_en_path** esta desarrollado con el lenguaje de programacion Rust lo cual implica que puede ser recompilado para todos los sistemas operativos que soporte Rust.


## Modo de uso
existe_en_path.exe <RUTA A CONSULTAR>

Devuelve en ERRORLEVEL la cantidad de ocurrencias que encontro, en caso de no encontrar ninguna devuelve 0.

**Ejemplos:**

```sh
existe_en_path "C:\WINDOWS\System32"
Resultado:
1 'C:\WINDOWS\system32'
```


Prueba.bat**
```sh
@echo off
existe_en_path "C:\Program Files\nodejs"
if %ERRORLEVEL%==0 set PATH=%PATH%;C:\Program Files\nodejs
```

Si se ejecuta sin una ruta, el codigo de salida es -1 (ERRORLEVEL==-1) y lista en orden alfabetico todas las rutas cargadas en la variable de entorno PATH.

```sh
existe_en_path

Resultado:
PATH ordenado:
---
C:\PROGRA~2\Borland\Delphi5\Bin
C:\PROGRA~2\Borland\Delphi5\Projects\Bpl
C:\PROGRA~2\Borland\vbroker\Bin
C:\PROGRA~2\Borland\vbroker\jre\Bin
C:\Program Files\Docker Toolbox
C:\Program Files\Git\cmd
C:\Program Files\Microsoft VS Code\bin
C:\Program Files\heroku\bin
C:\Program Files\nodejs
C:\Program Files\nodejs
C:\ProgramData\ComposerSetup\bin
C:\ProgramData\Oracle\Java\javapath
C:\ProgramData\chocolatey\bin
C:\Python39\
C:\Python39\Scripts\
C:\Users\Administrator\.cargo\bin
C:\Users\Administrator\AppData\Local\Microsoft\WindowsApps
C:\Users\Administrator\AppData\Local\Microsoft\WindowsApps
C:\Users\Administrator\AppData\Local\atom\bin
C:\Users\Administrator\AppData\Roaming\Composer\vendor\bin
C:\Users\Administrator\AppData\Roaming\npm
C:\Users\Administrator\AppData\Roaming\nvm
C:\Users\Administrator\AppData\Roaming\nvm
C:\WINDOWS
C:\WINDOWS\System32\OpenSSH\
C:\WINDOWS\System32\Wbem
C:\WINDOWS\System32\WindowsPowerShell\v1.0\
C:\WINDOWS\system32
C:\xampp\php
---
se listaron 33 rutas
```

```sh
existe_en_path "C:\Users\Administrator\AppData\Local\Microsoft\WindowsApps"
Resultado:
1 'C:\Users\Administrator\AppData\Local\Microsoft\WindowsApps'
2 'C:\Users\Administrator\AppData\Local\Microsoft\WindowsApps'
```

#### Para ejecutar
```sh
cargo run
```

#### Para construir en produccion:
```sh
cargo build --release
```





## License

MIT

**Free Software, Hell Yeah!**

[//]: # (These are reference links used in the body of this note and get stripped out when the markdown processor does its job. There is no need to format nicely because it shouldn't be seen. Thanks SO - http://stackoverflow.com/questions/4823468/store-comments-in-markdown-syntax)

   [dill]: <https://github.com/joemccann/dillinger>
   [git-repo-url]: <https://github.com/joemccann/dillinger.git>
   [john gruber]: <http://daringfireball.net>
   [df1]: <http://daringfireball.net/projects/markdown/>
   [markdown-it]: <https://github.com/markdown-it/markdown-it>
   [Ace Editor]: <http://ace.ajax.org>
   [node.js]: <http://nodejs.org>
   [Twitter Bootstrap]: <http://twitter.github.com/bootstrap/>
   [jQuery]: <http://jquery.com>
   [@tjholowaychuk]: <http://twitter.com/tjholowaychuk>
   [express]: <http://expressjs.com>
   [AngularJS]: <http://angularjs.org>
   [Gulp]: <http://gulpjs.com>

   [PlDb]: <https://github.com/joemccann/dillinger/tree/master/plugins/dropbox/README.md>
   [PlGh]: <https://github.com/joemccann/dillinger/tree/master/plugins/github/README.md>
   [PlGd]: <https://github.com/joemccann/dillinger/tree/master/plugins/googledrive/README.md>
   [PlOd]: <https://github.com/joemccann/dillinger/tree/master/plugins/onedrive/README.md>
   [PlMe]: <https://github.com/joemccann/dillinger/tree/master/plugins/medium/README.md>
   [PlGa]: <https://github.com/RahulHP/dillinger/blob/master/plugins/googleanalytics/README.md>
