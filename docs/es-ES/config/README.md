# Configuración

Para comenzar a configurar Starship, crea el siguiente archivo: `~/.config/starship.toml`.

```sh
mkdir -p ~/.config && starship print-config --default > ~/.config/starship.toml
```

Toda la configuración de Starship se realiza en este archivo [TOML](https://github.com/toml-lang/toml):

```toml
# Inserta una línea en blanco al inicio del prompt
add_newline = true

# Reemplaza el símbolo "❯" por "➜" en el prompt
[character]                            # El nombre del módulo que se está configurandoes "character"
success_symbol = "[➜](bold green)"     # El segmento "success_symbol" es reemplzado por "➜" con el color "bold green"

# Deshabilita el módulo "package", ocultándolo por completo del prompt
[package]
disabled = true
```

Puedes cambiar la ubicación por defecto del archivo de configuración con la variable de entorno `STARSHIP_CONFIG`:

```sh
export STARSHIP_CONFIG=~/.starship/config.toml
```

El equivalente en PowerShell (Windows) es añadir esta línea a tu `$PROFILE`:

```powershell
$ENV:STARSHIP_CONFIG = "$HOME\.starship\config.toml"
```

### Registros

Starship registra por defecto los mensajes de advertencia y error en un fichero con nombre `~/.cache/starship/session_${STARSHIP_SESSION_KEY}.log`, en el que la clave de sesión corresponde con una instancia de tu terminal. Esto, sin embargo, puede ser cambiado usando la variable de entorno `STARSHIP_CACHE`:

```sh
export STARSHIP_CACHE=~/.starship/cache
```

El equivalente en PowerShell (Windows) es añadir esta línea a tu `$PROFILE`:

```powershell
$ENV:STARSHIP_CACHE = "$HOME\AppData\Local\Temp"
```

### Terminología

**Módulo**: un componente en el promt que provee información basada en información contextual de tu sistema operativo. Por ejemplo, el módulo "nodejs" muestra la versión de NodeJS que tienes actualmente instalada en tu ordenador, si el directorio actual es un proyecto NodeJS.

**Variable**: subcomponentes más pequeños que contienen información proporcionada por el módulo. Por ejemplo, la variable "version" en el módulo "nodejs" contiene la versión actual de NodeJS.

Por convención, la mayoría de los módulos tienen un prefijo del color por defecto de la terminal (por ejemplo, `vía` en "nodejs") y un espacio vacío como sufijo.

### Cadenas de Formato

Es el formato con el que un módulo imprime todas sus variables. La mayoría de los módulos tienen una entrada llamada `format` que configura el formato de visualización del módulo. Se puede utilizar textos, variables y grupos de texto.

#### Variable

Una variable contiene un símbolo `$` seguido por el nombre de la variable. El nombre de una variable solo contiene letras, números y `_`.

Por ejemplo:

- `$version` es una cadena de formato con una variable llamada `version`.
- `$git_branch$git_commit` es un formato de cadena de texto con dos variables nombradas `git_branch` y `git_commit`.
- `$git_branch $git_commit` tiene las dos variables separadas por un espacio.

#### Grupo de Texto

Un grupo de texto se compone de dos partes diferentes.

La primera parte, que está encerrada en un `[]`, es una [cadena de formato](#format-strings). Se puede agregar textos, variables, o incluso grupos de texto anidados.

En la segunda parte, que está encerrada entre `()`, es una [cadena de estilo](#style-strings). Se puede utilizar el estilo de la primera parte.

Por ejemplo:

- `[en](bold red)` imprimirá una cadena `en` con texto en negrita color rojo.
- `[⌘ $version](bold green)` imprimirá un símbolo `⌘` seguido por el contenido de la variable `version`, con texto en negrita color verde.
- `[a [b](red) c](green)` imprimirá `a b c` con `b` en rojo, `a` y `c` en verde.

#### Cadenas de estilo

La mayoría de los módulos de starship permiten configurar los estilos de su cadenas texto. Esto se consigue con una entrada (normalmente llamada `style` - estilo) que no es más que un texto donde se especifica la configuración. A continuación mostramos algunos ejemplos de cadenas de estilo junto con su funcionalidad. Para más detalles sobre la sintaxis completa, consulta [la guía de configuración avanzada](/advanced-config/).

- `"fg:green bg:blue"` pone texto verde sobre un fondo azul
- `"bg:blue fg:bright-green"` pone texto verde claro sobre un fondo azul
- `"bold fg:27"` pone texto en negrita con [color ANSI](https://i.stack.imgur.com/KTSQa.png) 27
- `"underline bg:#bf5700"` subraya el texto sobre un fondo naranja oscuro
- `"bold italic fg:purple"` pone texto color morado, en negrita y cursiva
- `""` desactiva explícitamente cualquier estilo

Nótese que el estilo es similar a como se controlaría por el emulador de su terminal. Por ejemplo, algunos emuladores de terminal harán los colores más brillantes en lugar de más gruesos, y algunos temas usan los mismos valores para texto normal y colores brillantes. Además, para mostrar textos en cursiva tu terminal debe tener soporte para hacerlo.

#### Cadenas de Formato Condicional

Una cadena de formato condicional envuelto en `(` y `)` no se renderizará si todas las variables dentro están vacías.

Por ejemplo:

- `(@$region)` no mostrará nada si la variable `region` es `None`, de lo contrario `@` seguido por el valor de la región.
- `(algún texto)` siempre mostrará nada ya que no hay variables envueltas entre llaves.
- Cuando `$all` es un atajo para `\[$a$b\]`, `($all)` no mostrará nada solo si `$a` y `$b` ambos son `None`. Esto funciona igual que `(\[$a$b\] )`.

#### Caracteres de escape

Los siguientes símbolos tienen un uso especial en una cadena de formato. Si se quiere imprimir los siguientes símbolos, se tienen que escapar con una barra invertida (`\`).

- \$
- \\
- [
- ]
- (
- )

Se debe tener en cuenta que `toml` tiene [su propia sintaxis de escape](https://github.com/toml-lang/toml#user-content-string). Se recomienda usar una cadena literal (`''`) en la configuración. Si se desea utilizar una cadena básica (`""`), prestar atención para escapar la barra invertida `\`.

Por ejemplo, cuando se desea imprimir un símbolo `$` en una nueva línea, las siguientes configuraciones para la variable `format` son equivalentes:

```toml
# con cadena básica
format = "\n\\$"

# con cadena básica multilínea
format = """

\\$"""

# con cadena literal
format = '''

\$'''
```

## Prompt

Esta es la lista de opciones de configuración del prompt.

### Opciones

| Opción         | Por defecto                        | Descripción                                                                    |
| -------------- | ---------------------------------- | ------------------------------------------------------------------------------ |
| `format`       | [ver aquí](#default-prompt-format) | Configura el formato del prompt.                                               |
| `scan_timeout` | `30`                               | Tiempo de espera tras el que Starship escanea los archivos (en milisegundos).  |
| `add_newline`  | `true`                             | Inserta un línea en blanco entre las instrucciones del intérprete de comandos. |

### Ejemplo

```toml
# ~/.config/starship.toml

# Usar formato personalizado
format = """
[┌───────────────────>](bold green)
[│](bold green)$directory$rust$package
[└─>](bold green) """

# Espera 10 milisegundos para que starship revise los archivos del directorio actual.
scan_timeout = 10

# Deshabilita la línea en blanco al inicio del prompt
add_newline = false
```

### Formato por Defecto del Prompt

La varieble `format` por defecto se utiliza para definir el formato del prompt, si está vacía o `format` no se proporciona. El valor por defecto es el siguiente:

```toml
format = "$all"

# el cual es el equivalente a
format = """
$username\
$hostname\
$shlvl\
$kubernetes\
$directory\
$vcsh\
$git_branch\
$git_commit\
$git_state\
$git_status\
$hg_branch\
$docker_context\
$package\
$cmake\
$dart\
$dotnet\
$elixir\
$elm\
$erlang\
$golang\
$helm\
$java\
$julia\
$kotlin\
$nim\
$nodejs\
$ocaml\
$perl\
$php\
$purescript\
$python\
$ruby\
$rust\
$scala\
$swift\
$terraform\
$vagrant\
$zig\
$nix_shell\
$conda\
$memory_usage\
$aws\
$gcloud\
$openstack\
$env_var\
$crystal\
$custom\
$cmd_duration\
$line_break\
$lua\
$jobs\
$battery\
$time\
$status\
$shell\
$character"""
```

## AWS

El módulo `aws` muestra la región y el perfil actual de AWS. Éste se basa en las variables de entorno `AWS_REGION`, `AWS_DEFAULT_REGION`, y `AWS_PROFILE` del fichero `~/.aws/config`.

Cuando uses [aws-vault](https://github.com/99designs/aws-vault) el perfil se obtiene de la variable de entorno `AWS_VAULT`.

Cuando uses [awsu](https://github.com/kreuzwerker/awsu) el perfil se obtiene de la variable de entorno `AWSU_PROFILE`.

### Opciones

| Opción           | Por defecto                                         | Descripción                                                  |
| ---------------- | --------------------------------------------------- | ------------------------------------------------------------ |
| `format`         | `'on [$symbol($profile )(\($region\) )]($style)'` | El formato del módulo.                                       |
| `symbol`         | `"☁️ "`                                             | El símbolo que se muestra antes del perfil de AWS.           |
| `region_aliases` |                                                     | Tabla de alias de región para mostrar además del nombre AWS. |
| `style`          | `"bold yellow"`                                     | El estilo del módulo.                                        |
| `disabled`       | `false`                                             | Deshabilita el módulo `aws`.                                 |

### Variables

| Variable  | Ejemplo          | Descripción                            |
| --------- | ---------------- | -------------------------------------- |
| region    | `ap-northeast-1` | La región actual de AWS                |
| profile   | `astronauts`     | El perfil actual de AWS                |
| symbol    |                  | Refleja el valor de la opción `symbol` |
| style\* |                  | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplos

#### Mostrar todo

```toml
# ~/.config/starship.toml

[aws]
format = 'on [$symbol($profile )(\($region\) )]($style)'
style = "bold blue"
symbol = "🅰 "
[aws.region_aliases]
ap-southeast-2 = "au"
us-east-1 = "va"
```

#### Mostrar región

```toml
# ~/.config/starship.toml

[aws]
format = "en [$symbol$region]($style) "
style = "bold blue"
symbol = "🅰 "
[aws.region_aliases]
ap-southeast-2 = "au"
us-east-1 = "va"
```

#### Mostrar perfil

```toml
# ~/.config/starship.toml

[aws]
format = "en [$symbol$profile]($style) "
style = "bold blue"
symbol = "🅰 "
```

## Battery

El módulo `battery` muestra qué tan cargada está la batería del dispositivo y su estado de carga actual. El módulo sólo es visible cuando la batería del dispositivo está por debajo del 10%.

### Opciones

| Opción               | Por defecto                       | Descripción                                                              |
| -------------------- | --------------------------------- | ------------------------------------------------------------------------ |
| `full_symbol`        | `" "`                            | Se muestra cuando la batería está cargada.                               |
| `charging_symbol`    | `" "`                            | Se muestra cuando la batería se está cargando.                           |
| `discharging_symbol` | `" "`                            | Se muestra cuando la batería se está descargando.                        |
| `unknown_symbol`     | `" "`                            | El símbolo que se muestra cuando el estado de la batería es desconocido. |
| `empty_symbol`       | `" "`                            | El símbolo que se muestra cuando el estado de la batería está vacío.     |
| `format`             | `"[$symbol$percentage]($style) "` | El formato del módulo.                                                   |
| `display`            | [ver aquí](#battery-display)      | Define cuándo mostrar el indicador y el estilo.                          |
| `disabled`           | `false`                           | Desactiva el módulo `battery`.                                           |

### Ejemplo

```toml
# ~/.config/starship.toml

[battery]
full_symbol = "🔋 "
charging_symbol = "⚡️ "
discharging_symbol = "💀 "
```

### Indicador de batería

La opción de configuración de `display` se utiliza para definir cuándo se debe mostrar el indicador de batería (umbral) y cómo se ve (estilo). Si no se provee ningún valor para `display`. El valor por defecto es el siguiente:

```toml
[[battery.display]]
threshold = 10
style = "bold red"
```

#### Opciones

La opción `display` es un arreglo de la siguiente tabla.

| Opción      | Descripción                                                     |
| ----------- | --------------------------------------------------------------- |
| `threshold` | El umbral para la opción de visualización.                      |
| `style`     | El estilo usado cuando si la opción <0>display</0> está activa. |

#### Ejemplo

```toml
[[battery.display]]  # "bold red" cuando la carga está entre 0% y 10%
threshold = 10
style = "bold red"

[[battery.display]]  # "bold yellow" cuando la carga está entre 10% y 30%
threshold = 30
style = "bold yellow"

# cuando la carga está por encima del 30% el indicador no se mostrará

```

## Character

El módulo `character` muestra un caracter (normalmente una flecha) tras el texto que introduces en el terminal.

El carácter mostrará si el último comando funcionó o no. Se puede hacer de dos maneras:

- Cambiando el color (`red`/`green`)
- Cambiando la forma (`.`/`✖`)

Por defecto sólo cambia el color. Si también se quiere cambiar su forma, ver [este ejemplo](#with-custom-error-shape).

::: advertencia `error_symbol` no es compatible con el intérprete de comandos Elvish. :::

### Opciones

| Opción           | Por defecto         | Descripción                                                                                             |
| ---------------- | ------------------- | ------------------------------------------------------------------------------------------------------- |
| `format`         | `"$symbol "`        | La cadena de formato usada antes de la entrada de texto.                                                |
| `success_symbol` | `"[❯](bold green)"` | La cadena de formato usada antes de la entrada de texto si el comando anterior tuvo éxito.              |
| `error_symbol`   | `"[❯](bold red)"`   | La cadena de formato usada antes de la entrada de texto si el comando anterior falló.                   |
| `vicmd_symbol`   | `"[❮](bold green)"` | El cadena de formato antes de la entrada de texto si el intérprete de comandos está en modo vim normal. |
| `disabled`       | `false`             | Desactiva el módulo `character`.                                                                        |

### Variables

| Variable | Ejemplo | Descripción                                                    |
| -------- | ------- | -------------------------------------------------------------- |
| symbol   |         | Un espejo de `success_symbol`, `error_symbol` o `vicmd_symbol` |

### Ejemplos

#### Con formato de error personalizado

```toml
# ~/.config/starship.toml

[character]
success_symbol = "[➜](bold green) "
error_symbol = "[✗](bold red) "
```

#### Sin formato de error personalizado

```toml
# ~/.config/starship.toml

[character]
success_symbol = "[➜](bold green) "
error_symbol = "[➜](bold red) "
```

#### Con formato de vim personalizado

```toml
# ~/.config/starship.toml

[character]
vicmd_symbol = "[V](bold green) "
```

## CMake

El módulo `cmake` muestra la versión instalada de CMake. Por defecto el módulo se activará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `CMakeLists.txt`
- El directorio actual contiene un archivo `CMakeCache.txt`

### Opciones

| Opción              | Por defecto                            | Descripción                                      |
| ------------------- | -------------------------------------- | ------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`   | El formato del módulo.                           |
| `symbol`            | `"△ "`                                 | El símbolo usado antes de la versión de cmake.   |
| `detect_extensions` | `[]`                                   | Qué extensiones deben activar este módulo        |
| `detect_files`      | `["CMakeLists.txt", "CMakeCache.txt"]` | Qué nombres de archivo deben activar este módulo |
| `detect_folders`    | `[]`                                   | Qué carpetas deben activar este módulo           |
| `style`             | `"bold blue"`                          | El estilo del módulo.                            |
| `disabled`          | `false`                                | Deshabilita el módulo `cmake`.                   |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v3.17.3` | La versión de cmake                    |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

## Tiempo de Ejecución

El módulo `cmd_duration` muestra cuánto tiempo tardó el último comando en ejecutarse. El módulo se mostrará solo si el comando tomó más de dos segundos, o si el valor de configuración de `min_time` existe.

::: advertencia No utilizar la trampa DEBUG en Bash

Si estás usando Starship con `bash`, no uses `DEBUG` después de ejecutar `eval $(starship init $0)`, o el módulo **se romperá**.

:::

Los usuarios de Bash que necesiten la funcionalidad como preexec pueden usar el [framework bash_preexec de rcaloras](https://github.com/rcaloras/bash-preexec). Basta con definir los arreglos `preexec_functions` y `precmd_functions` antes de ejecutar `eval $(starship init $0)`, y luego proceder como siempre.

### Opciones

| Opción               | Por defecto                   | Descripción                                                            |
| -------------------- | ----------------------------- | ---------------------------------------------------------------------- |
| `min_time`           | `2_000`                       | Duración mínima para mostrar el tiempo de ejecución (en milisegundos)  |
| `show_milliseconds`  | `false`                       | Muestra la duración con precisión en milisegundos.                     |
| `format`             | `"took [$duration]($style) "` | El formato del módulo.                                                 |
| `style`              | `"bold yellow"`               | El estilo del módulo.                                                  |
| `disabled`           | `false`                       | Desactiva el módulo `cmd_duration`.                                    |
| `show_notifications` | `false`                       | Muestra notificaciones de escritorio cuando se complete el comando.    |
| `min_time_to_notify` | `45_000`                      | Duración mínima para mostrar el tiempo de ejecución (en milisegundos). |

::: consejo

Mostrar notificaciones de escritorio requiere que se construya Starship con soporte de `rust-notify`. Comprueba si tu Starship soporta notificaciones ejecutando `STARSHIP_LOG=debug starship module cmd_duration -d 60000` cuando `show_notifications` está establecido en `true`.

:::

### Variables

| Variable  | Ejemplo  | Descripción                                |
| --------- | -------- | ------------------------------------------ |
| duration  | `16m40s` | El tiempo que tardó en ejecutar el comando |
| style\* |          | Refleja el valor de la opción `style`      |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 500
format = "underwent [$duration](bold yellow)"
```

## Conda

El módulo `conda` muestra el entorno conda actual, si `$CONDA_DEFAULT_ENV` está configurado.

::: consejo

Esto no modifica el propio prompt de conda. En caso de querer suprimirlo, ejecuta `conda config --set changeps1 False`.

:::

### Opciones

| Opción              | Por defecto                            | Descripción                                                                                                                                                                                                             |
| ------------------- | -------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | El número de directorios a los que se debe truncar la variable de entorno, si el entorno fue creado usando `conda create -p [path]`. `0` significa sin truncamiento. Mirar también el módulo [`directory`](#directory). |
| `symbol`            | `"🅒 "`                                 | El símbolo usado antes del nombre del entorno.                                                                                                                                                                          |
| `style`             | `"bold green"`                         | El estilo del módulo.                                                                                                                                                                                                   |
| `format`            | `"via [$symbol$environment]($style) "` | El formato del módulo.                                                                                                                                                                                                  |
| `ignore_base`       | `true`                                 | Ignora el entorno `base` cuando se activa.                                                                                                                                                                              |
| `disabled`          | `false`                                | Desactiva el módulo `conda`.                                                                                                                                                                                            |

### Variables

| Variable    | Ejemplo      | Descripción                            |
| ----------- | ------------ | -------------------------------------- |
| environment | `astronauts` | El entorno conda actual                |
| symbol      |              | Refleja el valor de la opción `symbol` |
| style\*   |              | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[conda]
format = "[$symbol$environment](dimmed green) "
```

## Crystal

El módulo `crystal` muestra la versión actualmente instalada de Crystal. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un fichero `shard.yml`
- El directorio actual contiene un fichero `.cr`

### Opciones

| Opción              | Por defecto                          | Descripción                                          |
| ------------------- | ------------------------------------ | ---------------------------------------------------- |
| `symbol`            | `"🔮 "`                               | Símbolo usado antes de la versión de Crystal.        |
| `style`             | `"bold red"`                         | El estilo del módulo.                                |
| `detect_extensions` | `["cr"]`                             | Qué extensiones deberían activar este módulo.        |
| `detect_files`      | `["shard.yml"]`                      | Qué nombres de archivo deberían activar este módulo. |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.           |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                               |
| `disabled`          | `false`                              | Deshabilita el módulo `crystal`.                     |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v0.32.1` | La versión de `crystal`                |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[crystal]
format = "via [✨ $version](bold blue) "
```

## Dart

El módulo `dart` muestra la versión actualmente instalada de Dart. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo con la extensión `.dart`
- El directorio actual contiene un directorio `.dart_tool`
- El directorio actual contiene un archivo `pubspec.yaml`, `pubspec.yml` o `pubspec.lock`

### Opciones

| Opción              | Por defecto                                       | Descripción                                             |
| ------------------- | ------------------------------------------------- | ------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`              | El formato del módulo.                                  |
| `symbol`            | `"🎯 "`                                            | Una cadena de formato que representa el símbolo de Dart |
| `detect_extensions` | `['dart']`                                        | Qué extensiones deberían activar este módulo.           |
| `detect_files`      | `["pubspec.yaml", "pubspec.yml", "pubspec.lock"]` | Qué nombres de archivo deberían activar este módulo.    |
| `detect_folders`    | `[".dart_tool"]`                                  | Qué carpetas deberían activar este módulo.              |
| `style`             | `"bold blue"`                                     | El estilo del módulo.                                   |
| `disabled`          | `false`                                           | Deshabilita el módulo `dart`.                           |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v2.8.4` | La versión de `dart`                   |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[dart]
format = "via [🔰 $version](bold red) "
```

## Directory

El módulo `directory` muestra la ruta a tu directorio actual, truncado a tres carpetas padres. Tu directorio se truncará a la raíz del repositorio git en el que te encuentres.

Cuando usas el estilo fish de la opción pwd, en lugar de ocultar la ruta truncada, verás una versión acortada del nombre de cada directorio basada en el número que activaste para la opción.

Por ejemplo, dado `~/Dev/Nix/nixpkgs/pkgs` donde `nixpkgs` es la raíz del repositorio y la opción establecida a `1`. Ahora verás `~/D/N/nixpkgs/pkgs`, mientras que antes habría sido `nixpkgs/pkgs`.

### Opciones

| Opción              | Por defecto                                        | Descripción                                                                    |
| ------------------- | -------------------------------------------------- | ------------------------------------------------------------------------------ |
| `truncation_length` | `3`                                                | El número de directorios padre a los que se debe truncar el directorio actual. |
| `truncate_to_repo`  | `true`                                             | Trunca o no hasta la raíz del repositorio git en el que estés.                 |
| `format`            | `"[$path]($style)[$read_only]($read_only_style) "` | El formato del módulo.                                                         |
| `style`             | `"bold cyan"`                                      | El estilo del módulo.                                                          |
| `disabled`          | `false`                                            | Desactiva el módulo `directory`.                                               |
| `read_only`         | `"🔒"`                                              | El símbolo que indica si el directorio actual es de sólo lectura.              |
| `read_only_style`   | `"red"`                                            | El estilo para el símbolo de sólo lectura.                                     |
| `truncation_symbol` | `""`                                               | El símbolo a prefijar a las rutas truncadas. ej: "…/"                          |
| `home_symbol`       | `"~"`                                              | El símbolo que indica el directorio personal.                                  |

<details>
<summary>Este módulo tiene algunas opciones avanzadas de configuración que controlan cómo se muestra el directorio.</summary>

| Opciones avanzadas          | Por defecto | Descripción                                                                                                                                                                                                                  |
| --------------------------- | ----------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `substitutions`             |             | Una tabla de sustituciones que se deben hacer a la ruta.                                                                                                                                                                     |
| `fish_style_pwd_dir_length` | `0`         | El número de caracteres a usar al aplicar la lógica de ruta pwd del intérprete de comandos de Fish.                                                                                                                          |
| `use_logical_path`          | `true`      | Si `true` renderiza la ruta lógica originada desde el intérprete de comandos a través de `PWD` o `--logical-path`. Si `false` en su lugar renderiza la ruta física del sistema de archivos con enlaces simbólicos resueltos. |

`substitutions` permite definir reemplazos arbitrarios para cadenas literales que ocurren en la ruta, por ejemplo prefijos largos de red o directorios de desarrollo (p. ej. Java). Ten en cuenta que esto desactivará el estilo PWD de fish.

```toml
[directory.substitutions]
"/Volumes/network/path" = "/net"
"src/com/long/java/path" = "mypath"
```

`fish_style_pwd_dir_length` interactúa con las opciones de truncamiento estándar de una manera que puede sorprenderse primero: si no es cero, los componentes de la ruta que normalmente se truncarían se muestran con esa cantidad de caracteres. Por ejemplo, la ruta `/built/this/city/on/rock/and/roll`, que normalmente se mostraría como `rock/and/roll`, se mostraría como `/b/t/c/o/rock/and/roll` con `fish_style_pwd_dir_length = 1`--los componentes de ruta que normalmente se eliminarían se muestran con un solo carácter. Para `fish_style_pwd_dir_length = 2`, sería `/bu/th/ci/on/rock/and/roll`.

</details>

### Variables

| Variable  | Ejemplo               | Descripción                           |
| --------- | --------------------- | ------------------------------------- |
| path      | `"D:/Projects"`       | La ruta de directorio actual          |
| style\* | `"black bold dimmed"` | Refleja el valor de la opción `style` |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
truncation_symbol = "…/"
```

## Docker Context

El módulo `docker_context` muestra el [contexto de Docker](https://docs.docker.com/engine/context/working-with-contexts/) actualmente activo si no está establecido en `por defecto`.

### Opciones

| Opción              | Por defecto                                                   | Descripción                                                                                                               |
| ------------------- | ------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol$context]($style) "`                            | El formato del módulo.                                                                                                    |
| `symbol`            | `"🐳 "`                                                        | El símbolo usado antes de mostrar el contexto de Docker.                                                                  |
| `only_with_files`   | `true`                                                        | Mostrar solo cuando haya una coincidencia                                                                                 |
| `detect_extensions` | `[]`                                                          | Qué extensiones deberían activar este módulo (necesita que `solly_with_files` sea verdadero, con un valor "true").        |
| `detect_files`      | `["docker-compose.yml", "docker-compose.yaml", "Dockerfile"]` | Qué nombres de archivo deberían activar este módulo (necesita que `solly_with_files` sea verdadero, con un valor "true"). |
| `detect_folders`    | `[]`                                                          | Qué carpetas deberían activar este módulo (necesita que `solly_with_files` sea verdadero, con un valor "true").           |
| `style`             | `"blue bold"`                                                 | El estilo del módulo.                                                                                                     |
| `disabled`          | `false`                                                       | Deshabilita el módulo `docker_context`.                                                                                   |

### Variables

| Variable  | Ejemplo        | Descripción                            |
| --------- | -------------- | -------------------------------------- |
| context   | `test_context` | El contexto actual de docker           |
| symbol    |                | Refleja el valor de la opción `symbol` |
| style\* |                | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[docker_context]
format = "via [🐋 $context](blue bold)"
```

## Dotnet

El módulo `dotnet` muestra la versión relevante de .NET Core SDK para el directorio actual. Si el SDK ha sido anclado en el directorio actual, se mostrará la versión fijada. De lo contrario, el módulo muestra la última versión instalada del SDK.

Por defecto, este módulo solo se mostrará en tu prompt cuando uno o más de de los siguientes archivos estén presentes en el directorio actual:

- `global.json`
- `project.json`
- `Directory.Build.props`
- `Directory.Build.targets`
- `Packages.props`
- `*.sln`
- `*.csproj`
- `*.fsproj`
- `*.xproj`

También necesitarás tener instalado .NET Core SDK para poder usarlo correctamente.

Internamente, este módulo utiliza su propio mecanismo para la detección de versiones. Normalmente es el doble de rápido que ejecutar `dotnet --version`, pero puede mostrar una versión incorrecta si tu proyecto .NET tiene un diseño de directorio inusual. Si la precisión es más importante que la velocidad, puede desactivar el mecanismo estableciendo `heuristic = false` en las opciones del módulo.

El módulo también mostrará el Target Framework Moniker ([https://docs.microsoft. om/es/dotnet/standard/frameworks#supported-target-framework-versions](https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-framework-versions)) cuando exista un archivo csproj en el directorio actual.

### Opciones

| Opción              | Por defecto                                                                                             | Descripción                                                               |
| ------------------- | ------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"[$symbol($version )(🎯 $tfm )]($style)"`                                                               | El formato del módulo.                                                    |
| `symbol`            | `".NET "`                                                                                               | Símbolo usado antes de mostrar la versión de .NET                         |
| `heuristic`         | `true`                                                                                                  | Usa una detección de versiones más rápida para mantener a starship veloz. |
| `detect_extensions` | `["sln", "csproj", "fsproj", "xproj"]`                                                                  | Qué extensiones deberían activar este módulo.                             |
| `detect_files`      | `["global.json", "project.json", "Directory.Build.props", "Directory.Build.targets", "Packages.props"]` | Qué nombres de archivo deberían activar este módulo.                      |
| `detect_folders`    | `[]`                                                                                                    | Qué carpetas deberían activar estos módulos.                              |
| `style`             | `"bold blue"`                                                                                           | El estilo del módulo.                                                     |
| `disabled`          | `false`                                                                                                 | Deshabilita el módulo `dotnet`.                                           |

### Variables

| Variable  | Ejemplo          | Descripción                                                     |
| --------- | ---------------- | --------------------------------------------------------------- |
| version   | `v3.1.201`       | La version del sdk de `dotnet`                                  |
| tfm       | `netstandard2.0` | El Target Framework Moniker al que se dirige el proyecto actual |
| symbol    |                  | Refleja el valor de la opción `symbol`                          |
| style\* |                  | Refleja el valor de la opción `style`                           |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[dotnet]
symbol = "🥅 "
style = "green"
heuristic = false
```

## Elixir

El módulo `elixir` muestra la version instalada actualmente de Elixir y Erlang/OTP. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `mix.exs`.

### Opciones

| Opción              | Por defecto                                                 | Descripción                                                    |
| ------------------- | ----------------------------------------------------------- | -------------------------------------------------------------- |
| `symbol`            | `"💧 "`                                                      | El símbolo usado antes de mostrar la version de Elixir/Erlang. |
| `detect_extensions` | `[]`                                                        | Qué extensiones deberían activar este módulo.                  |
| `detect_files`      | `["mix.exs"]`                                               | Qué nombres de archivo deberían activar este módulo.           |
| `detect_folders`    | `[]`                                                        | Qué carpetas deberían activar estos módulos.                   |
| `style`             | `"bold purple"`                                             | El estilo del módulo.                                          |
| `format`            | `'via [$symbol($version \(OTP $otp_version\) )]($style)'` | El formato para el módulo de elixir.                           |
| `disabled`          | `false`                                                     | Deshabilita el módulo `elixir`.                                |

### Variables

| Variable    | Ejemplo | Descripción                            |
| ----------- | ------- | -------------------------------------- |
| version     | `v1.10` | La version de `elixir`                 |
| otp_version |         | La version de otp de `elixir`          |
| symbol      |         | Refleja el valor de la opción `symbol` |
| style\*   |         | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[elixir]
symbol = "🔮 "
```

## Elm

El módulo `elm` muestra la versión actualmente instalada de Elm. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un fichero `elm.json`
- El directorio actual contiene un fichero `elm-package.json`
- El directorio actual contiene un archivo `.elm-version`
- El directorio actual contiene una carpeta `elm-stuff`
- El directorio actual contiene archivos `*.elm`

### Opciones

| Opción              | Por defecto                                        | Descripción                                             |
| ------------------- | -------------------------------------------------- | ------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`               | El formato del módulo.                                  |
| `symbol`            | `"🌳 "`                                             | Una cadena de formato que representa el símbolo de Elm. |
| `detect_extensions` | `["elm"]`                                          | Qué extensiones deberían activar este módulo.           |
| `detect_files`      | `["elm.json", "elm-package.json", ".elm-version"]` | Qué nombres de archivo deberían activar este módulo.    |
| `detect_folders`    | `["elm-stuff"]`                                    | Qué carpetas deberían activar estos módulos.            |
| `style`             | `"cyan bold"`                                      | El estilo del módulo.                                   |
| `disabled`          | `false`                                            | Deshabilita el módulo `elm`.                            |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v0.19.1` | La versión de `elm`                    |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[elm]
format = "via [ $version](cyan bold) "
```

## Variable de entorno

El módulo `env_var` muestra el valor actual de una variable de entorno seleccionada. El módulo se mostrará sólo si se cumplen cualquiera de las siguientes condiciones:

- La opción de configuración de `variable` coincide con una variable de entorno existente
- La opción de configuración de `variable` no está definida, pero la opción de configuración `predeterminada` se encuentra

### Opciones

| Opción        | Por defecto                    | Descripción                                                                            |
| ------------- | ------------------------------ | -------------------------------------------------------------------------------------- |
| `symbol`      |                                | El símbolo usado antes de mostrar el valor de la variable.                             |
| `variable`    |                                | La variable de entorno a mostrar.                                                      |
| `por defecto` |                                | El valor por defecto que se mostrará cuando la variable seleccionada no está definida. |
| `format`      | `"with [$env_value]($style) "` | El formato del módulo.                                                                 |
| `disabled`    | `false`                        | Desactiva el módulo `env_var`.                                                         |

### Variables

| Variable  | Ejemplo                               | Descripción                                 |
| --------- | ------------------------------------- | ------------------------------------------- |
| env_value | `Windows NT` (si _variable_ es `$OS`) | El valor de entorno de la opción `variable` |
| symbol    |                                       | Refleja el valor de la opción `symbol`      |
| style\* | `black bold dimmed`                   | Refleja el valor de la opción `style`       |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[env_var]
variable = "SHELL"
default = "unknown shell"
```

## Erlang

El módulo `erlang` muestra la versión instalada de Erlang/OTP. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un fichero `rebar.config`.
- El directorio actual contiene un fichero `erlang.mk`.

### Opciones

| Opción              | Por defecto                          | Descripción                                             |
| ------------------- | ------------------------------------ | ------------------------------------------------------- |
| `symbol`            | `" "`                               | El símbolo usado antes de mostrar la versión de Erlang. |
| `style`             | `"bold red"`                         | El estilo del módulo.                                   |
| `detect_extensions` | `[]`                                 | Qué extensiones deberían activar este módulo.           |
| `detect_files`      | `["rebar.config", "elang.mk"]`       | Qué nombres de archivo deberían activar este módulo.    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar estos módulos.            |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                  |
| `disabled`          | `false`                              | Deshabilita el módulo `erlang`.                         |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v22.1.3` | La versión de `erlang`                 |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[erlang]
format = "via [e $version](bold red) "
```

## Gcloud

El módulo `gcloud` muestra la configuración actual para el CLI de [`gcloud`](https://cloud.google.com/sdk/gcloud). Esto se basa en el archivo `~/.config/gcloud/active_config`, el archivo `~/.config/gcloud/configurations/config_{CONFIG NAME}` y la variable de entorno `CLOUDSDK_CONFIG`.

### Opciones

| Opción           | Por defecto                                      | Descripción                                                |
| ---------------- | ------------------------------------------------ | ---------------------------------------------------------- |
| `format`         | `'on [$symbol$account(\($region\))]($style) '` | El formato del módulo.                                     |
| `symbol`         | `"☁️ "`                                          | El símbolo usado antes de mostrar el perfil actual de GCP. |
| `region_aliases` |                                                  | Tabla de alias de región a mostrar además del nombre GCP.  |
| `style`          | `"bold blue"`                                    | El estilo del módulo.                                      |
| `disabled`       | `false`                                          | Desactiva el módulo `gcloud`.                              |

### Variables

| Variable  | Ejemplo           | Descripción                                                                   |
| --------- | ----------------- | ----------------------------------------------------------------------------- |
| region    | `us-central1`     | La región GCP actual                                                          |
| account   | `foo@example.com` | El perfil actual de GCP                                                       |
| project   |                   | El proyecto GCP actual                                                        |
| active    | `por defecto`     | El nombre de configuración activo escrito en `~/.config/gcloud/active_config` |
| symbol    |                   | Refleja el valor de la opción `symbol`                                        |
| style\* |                   | Refleja el valor de la opción `style`                                         |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplos

#### Mostrar cuenta y proyecto

```toml
# ~/.config/starship.toml

[gcloud]
format = 'on [$symbol$account(\($project\))]($style) '
```

#### Mostrar solo el nombre de la configuración activa

```toml
# ~/.config/starship.toml

[gcloud]
format = "[$symbol$active]($style) "
style = "bold yellow"
```

#### Mostrar los alias de cuenta y región

```toml
# ~/.config/starship.toml

[gcloud]
symbol = "️🇬️ "
[gcloud.region_aliases]
us-central1 = "uc1"
asia-northeast1 = "an1"
```

## Git Branch

El módulo `git_branch` muestra la rama activa del repositorio en su directorio actual.

### Opciones

| Opción               | Por defecto                      | Descripción                                                                                             |
| -------------------- | -------------------------------- | ------------------------------------------------------------------------------------------------------- |
| `always_show_remote` | `false`                          | Muestra el nombre de la rama de seguimiento remoto, incluso si es igual al nombre de la rama local.     |
| `format`             | `"on [$symbol$branch]($style) "` | El formato del módulo. Use `"$branch"` para referirse al nombre de la rama actual.                      |
| `symbol`             | `" "`                           | Una cadena de formato que representa el símbolo de la rama git.                                         |
| `style`              | `"bold purple"`                  | El estilo del módulo.                                                                                   |
| `truncation_length`  | `2^63 - 1`                       | Trunca una rama git a grafemas `N`.                                                                     |
| `truncation_symbol`  | `"…"`                            | El símbolo usado para indicar que un nombre de rama fue truncado. Puedes usar `""` para ningún símbolo. |
| `only_attached`      | `false`                          | Mostrar solo el nombre de la rama cuando no esté en un estado `HEAD`.                                   |
| `disabled`           | `false`                          | Desactiva el módulo `git_branch`.                                                                       |

### Variables

| Variable      | Ejemplo  | Descripción                                                                                                    |
| ------------- | -------- | -------------------------------------------------------------------------------------------------------------- |
| branch        | `master` | El nombre de la rama actual, vuelve a `HEAD` si no hay ninguna rama actual (por ejemplo, git detached `HEAD`). |
| remote_name   | `origin` | El nombre remoto.                                                                                              |
| remote_branch | `master` | El nombre de la rama rastreada en `remote_name`.                                                               |
| symbol        |          | Refleja el valor de la opción `symbol`                                                                         |
| style\*     |          | Refleja el valor de la opción `style`                                                                          |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[git_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
```

## Git commit

El módulo `git_commit` muestra el hash de la confirmación actual y también la etiqueta (si existe) del repositorio en su directorio actual.

### Opciones

| Opción               | Por defecto                                            | Descripción                                                                            |
| -------------------- | ------------------------------------------------------ | -------------------------------------------------------------------------------------- |
| `commit_hash_length` | `7`                                                    | La longitud del hash del commit de git mostrado.                                       |
| `format`             | `"[\\($hash\\)]($style) [\\($tag\\)]($style)"` | El formato del módulo.                                                                 |
| `style`              | `"bold green"`                                         | El estilo del módulo.                                                                  |
| `only_detached`      | `true`                                                 | Mostrar solo el hash de la confirmación de git cuando esté en estado "detached `HEAD`" |
| `tag_disabled`       | `true`                                                 | Deshabilita mostrar información de etiquetas en el módulo `git_commit`.                |
| `tag_symbol`         | `"🏷 "`                                                 | Símbolo de etiqueta prefijando la información mostrada                                 |
| `disabled`           | `false`                                                | Desactiva el módulo `git_commit`.                                                      |

### Variables

| Variable  | Ejemplo   | Descripción                           |
| --------- | --------- | ------------------------------------- |
| hash      | `b703eb3` | El hash actual del commit de git      |
| style\* |           | Refleja el valor de la opción `style` |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
tag_symbol = "🔖 "
```

## Git state

El módulo `git_state` se mostrará en directorios que son parte de un repositorio git, y donde hay una operación en curso, tales como: _REBASING_, _BISECTING_, etc. Si hay información de progreso (por ejemplo, REBASING 3/10), esa información será mostrada también.

### Opciones

| Opción         | Por defecto                                                     | Descripción                                                                                         |
| -------------- | --------------------------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| `rebase`       | `"REBASING"`                                                    | Una cadena de formato que se muestra cuando un `rebase` está en progreso.                           |
| `merge`        | `"MERGING"`                                                     | Una cadena de formato que se muestra cuando un `merge` está en progreso.                            |
| `revert`       | `"REVERTING"`                                                   | Una cadena de formato mostrada cuando un `revert` está en progreso.                                 |
| `cherry_pick`  | `"CHERRY-PICKING"`                                              | Una cadena de formato que se muestra cuando un `cherry-pick` está en progreso.                      |
| `bisect`       | `"BISECTING"`                                                   | Una cadena de formato que se muestra cuando un `bisect` está en progreso.                           |
| `am`           | `"AM"`                                                          | Una cadena de formato que se muestra cuando un `apply-mailbox` (`git am`) está en progeso.          |
| `am_or_rebase` | `"AM/REBASE"`                                                   | Una cadena de formato que se muestra cuando un ambiguo `apply-builbox` o `rebase` está en progreso. |
| `style`        | `"bold yellow"`                                                 | El estilo del módulo.                                                                               |
| `format`       | `'\([$state( $progress_current/$progress_total)]($style)\) '` | El formato del módulo.                                                                              |
| `disabled`     | `false`                                                         | Desactiva el módulo `git_state`.                                                                    |

### Variables

| Variable         | Ejemplo    | Descripción                           |
| ---------------- | ---------- | ------------------------------------- |
| state            | `REBASING` | El estado actual del repositorio      |
| progress_current | `1`        | El progreso de la operación actual    |
| progress_total   | `2`        | El progreso total de la operación     |
| style\*        |            | Refleja el valor de la opción `style` |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[git_state]
format = '[\($state( $progress_current of $progress_total)\)]($style) '
cherry_pick = "[🍒 PICKING](bold red)"
```

## Git status

El módulo `git_status` muestra símbolos que representan el estado del repositorio en su directorio actual.

### Opciones

| Opción       | Por defecto                                     | Descripción                              |
| ------------ | ----------------------------------------------- | ---------------------------------------- |
| `format`     | `'([\[$all_status$ahead_behind\]]($style) )'` | El formato por defecto para `git_status` |
| `conflicted` | `"="`                                           | Esta rama tiene conflictos de fusión.    |
| `ahead`      | `"⇡"`                                           | El formato de `ahead`                    |
| `behind`     | `"⇣"`                                           | El formato de `behind`                   |
| `diverged`   | `"⇕"`                                           | El formato de `diverged`                 |
| `untracked`  | `"?"`                                           | El formato de `untracked`                |
| `stashed`    | `"$"`                                           | El formato de `stashed`                  |
| `modified`   | `"!"`                                           | El formato de `modified`                 |
| `staged`     | `"+"`                                           | El formato de `staged`                   |
| `renamed`    | `"»"`                                           | El formato de `renamed`                  |
| `deleted`    | `"✘"`                                           | El formato de `deleted`                  |
| `style`      | `"bold red"`                                    | El estilo del módulo.                    |
| `disabled`   | `false`                                         | Desactiva el módulo `git_status`.        |

### Variables

Las siguientes variables se pueden utilizar en `format`:

| Variable       | Descripción                                                                                              |
| -------------- | -------------------------------------------------------------------------------------------------------- |
| `all_status`   | Atajo para `$conflicted$stashed$deleted$renamed$modified$staged$untracked`                               |
| `ahead_behind` | Muestra la cadena de formato de `diverged` `ahead` o `behind` basado en el estado actual del repositorio |
| `conflicted`   | Muestra `conflicted` cuando esta rama tiene conflictos de fusión.                                        |
| `untracked`    | Muestra `untracked` cuando hay archivos sin rastrear en el directorio de trabajo.                        |
| `stashed`      | Muestra `stashed` cuando existe un "stash" para el repositorio local.                                    |
| `modified`     | Muestra `modified` cuando hay modificaciones de archivo en el directorio de trabajo.                     |
| `staged`       | Muestra `staged` cuando se ha añadido un nuevo archivo al área de "stash".                               |
| `renamed`      | Muestra `renamed` cuando un archivo renombrado ha sido añadido al área de "stash".                       |
| `deleted`      | Muestra `deleted` cuando un archivo ha sido añadido al área de "stash".                                  |
| style\*      | Refleja el valor de la opción `style`                                                                    |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

Las siguientes variables pueden ser usadas en `diverged`:

| Variable       | Descripción                                             |
| -------------- | ------------------------------------------------------- |
| `ahead_count`  | Número de commits por delante de la rama de seguimiento |
| `behind_count` | Número de commits detrás de la rama de seguimiento      |

Las siguientes variales pueden ser usadas en `conflicted`, `ahead`, `behind`, `untracked`, `stashed`, `modified`, `staged`, `renamed` and `deleted`:

| Variable | Descripción                   |
| -------- | ----------------------------- |
| `count`  | Muestra el número de archivos |

### Ejemplo

```toml
# ~/.config/starship.toml

[git_status]
conflicted = "🏳"
ahead = "🏎💨"
behind = "😰"
diverged = "😵"
untracked = "🤷‍"
stashed = "📦"
modified = "📝"
staged = '[++\($count\)](green)'
renamed = "👅"
deleted = "🗑"
```

Mostrar el recuento delante/detrás de la rama que está siendo rastreada

```toml
# ~/.config/starship.toml

[git_status]
ahead = "⇡${count}"
diverged = "⇕⇡${ahead_count}⇣${behind_count}"
behind = "⇣${count}"
```

## Golang

El módulo `golang` muestra la versión actualmente instalada de Golang. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un fichero `go.mod`
- El directorio actual contiene un fichero `go.sum`
- El directorio actual contiene un fichero `glide.yaml`
- El directorio actual contiene un archivo `Gopkg.yml`
- El directorio actual contiene un archivo `Gopkg.lock`
- El directorio actual contiene un archivo `.go-version`
- El directorio actual contiene un directorio `Godeps`
- El directorio actual contiene un archivo con la extensión `.go`

### Opciones

| Opción              | Por defecto                                                                    | Descripción                                            |
| ------------------- | ------------------------------------------------------------------------------ | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`                                           | El formato del módulo.                                 |
| `symbol`            | `"🐹 "`                                                                         | Una cadena de formato que representa el símbolo de Go. |
| `detect_extensions` | `["go"]`                                                                       | Qué extensiones deberían activar este módulo.          |
| `detect_files`      | `["go.mod", "go.sum", "glide.yaml", "Gopkg.yml", "Gopkg.lock", ".go-version"]` | Qué nombres de archivo deberían activar este módulo.   |
| `detect_folders`    | `["Godeps"]`                                                                   | Qué carpetas deberían activar este módulo.             |
| `style`             | `"bold cyan"`                                                                  | El estilo del módulo.                                  |
| `disabled`          | `false`                                                                        | Deshabilita el módulo de `golang`.                     |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v1.12.1` | La versión de `go`                     |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[golang]
format = "via [🏎💨 $version](bold cyan) "
```

## Helm

El módulo `helm` muestra la versión instalada de Helm. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un fichero `helmfile.yaml`
- El directorio actual contiene un archivo `Chart.yaml`

### Opciones

| Opción              | Por defecto                          | Descripción                                              |
| ------------------- | ------------------------------------ | -------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                   |
| `detect_extensions` | `[]`                                 | Qué extensiones deberían activar este módulo.            |
| `detect_files`      | `["helmfile.yaml", "Chart.yaml"]`    | Qué nombres de archivo deberían activar este módulo.     |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar estos módulos.             |
| `symbol`            | `"⎈ "`                               | Una cadena de formato que representa el símbolo de Helm. |
| `style`             | `"bold white"`                       | El estilo del módulo.                                    |
| `disabled`          | `false`                              | Deshabilita el módulo `helm`.                            |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v3.1.1` | La versión de `helm`                   |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[helm]
format = "via [⎈ $version](bold white) "
```

## Hostname

El módulo `hostname` muestra el nombre de host del sistema.

### Opciones

| Opción     | Por defecto                 | Descripción                                                                                                                                                       |
| ---------- | --------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ssh_only` | `true`                      | Mostrar sólo el nombre de host cuando esté conectado a una sesión SSH.                                                                                            |
| `trim_at`  | `"."`                       | Cadena en la que el nombre de host se corta, después de la primera partida. `"."` se detendrá después del primer punto. `""` deshabilitará cualquier truncamiento |
| `format`   | `"[$hostname]($style) in "` | El formato del módulo.                                                                                                                                            |
| `style`    | `"bold dimmed green"`       | El estilo del módulo.                                                                                                                                             |
| `disabled` | `false`                     | Desactiva el módulo `hostname`.                                                                                                                                   |

### Variables

| Variable  | Ejemplo | Descripción                            |
| --------- | ------- | -------------------------------------- |
| symbol    |         | Refleja el valor de la opción `symbol` |
| style\* |         | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
format =  "on [$hostname](bold red) "
trim_at = ".companyname.com"
disabled = false
```

## Java

El módulo `java` muestra la versión actualmente instalada de Java. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `pom.xml`, `build.gradle.kts`, `build.sbt`, `.java-version`, `.deps.edn`, `project.clj`, o `build.boot`
- El directorio actual contiene un archivo con la extensión `.java`, `.class`, `.gradle` o `.jar`, `.clj` o `.cljc`

### Opciones

| Opción              | Por defecto                                                                                               | Descripción                                             |
| ------------------- | --------------------------------------------------------------------------------------------------------- | ------------------------------------------------------- |
| `format`            | `"via [${symbol}(${version} )]($style)"`                                                                  | El formato del módulo.                                  |
| `detect_extensions` | `["java", "class", "gradle", "jar", "cljs", "cljc"]`                                                      | Qué extensiones deberían activar este módulo.           |
| `detect_files`      | `["pom.xml", "build.gradle.kts", "build.sbt", ".java-version", ".deps.edn", "project.clj", "build.boot"]` | Qué nombres de archivo deberían activar este módulo.    |
| `detect_folders`    | `[]`                                                                                                      | Qué carpetas deberían activar estos módulos.            |
| `symbol`            | `"☕ "`                                                                                                    | Una cadena de formato que representa el símbolo de Java |
| `style`             | `"red dimmed"`                                                                                            | El estilo del módulo.                                   |
| `disabled`          | `false`                                                                                                   | Deshabilita el módulo `java`.                           |

### Variables

| Variable  | Ejemplo | Descripción                            |
| --------- | ------- | -------------------------------------- |
| version   | `v14`   | La versión de `java`                   |
| symbol    |         | Refleja el valor de la opción `symbol` |
| style\* |         | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[java]
symbol = "🌟 "
```

## Jobs

El módulo `jobs` muestra el número actual de tareas en ejecución. El módulo se mostrará sólo si hay tareas en segundo plano ejecutándose. El módulo mostrará el número de tareas en ejecución si hay más de 1 tarea o más que el valor configurado de `threshold`, si existe.

::: aviso

Este módulo no está soportado en tcsh.

:::

### Opciones

| Opción      | Por defecto                   | Descripción                                               |
| ----------- | ----------------------------- | --------------------------------------------------------- |
| `threshold` | `1`                           | Muestra el número de tareas si se exceden.                |
| `format`    | `"[$symbol$number]($style) "` | El formato del módulo.                                    |
| `symbol`    | `"✦"`                         | Una cadena de formato que representa el número de tareas. |
| `style`     | `"bold blue"`                 | El estilo del módulo.                                     |
| `disabled`  | `false`                       | Desactiva el módulo `jobs`.                               |

### Variables

| Variable  | Ejemplo | Descripción                            |
| --------- | ------- | -------------------------------------- |
| number    | `1`     | El número de tareas                    |
| symbol    |         | Refleja el valor de la opción `symbol` |
| style\* |         | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[jobs]
symbol = "+ "
threshold = 4
```

## Julia

El módulo `julia` muestra la versión actualmente instalada de Julia. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `Project.toml`
- El directorio actual contiene un archivo `Manifest.toml`
- El directorio actual contiene un archivo con la extensión `.jl`

### Opciones

| Opción              | Por defecto                          | Descripción                                               |
| ------------------- | ------------------------------------ | --------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                    |
| `detect_extensions` | `["jl"]`                             | Qué extensiones deberían activar este módulo.             |
| `detect_files`      | `["Project.toml", "Manifest.toml"]`  | Qué nombres de archivo deberían activar este módulo.      |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar estos módulos.              |
| `symbol`            | `"ஃ "`                               | Una cadena de formato que representa el símbolo de Julia. |
| `style`             | `"bold purple"`                      | El estilo del módulo.                                     |
| `disabled`          | `false`                              | Deshabilita el módulo `julia`.                            |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v1.4.0` | La versión de `julia`                  |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[julia]
symbol = "∴ "
```

## Kotlin

El módulo `kotlin` muestra la versión instalada de Kotlin. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `.kt` o `.kts`

### Opciones

| Opción              | Por defecto                          | Descripción                                                             |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                                  |
| `detect_extensions` | `["kt", "kts"]`                      | Qué extensiones deberían activar este módulo.                           |
| `detect_files`      | `[]`                                 | Qué nombres de archivo deberían activar este módulo.                    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar estos módulos.                            |
| `symbol`            | `"🅺 "`                               | Una cadena de formato que representa el símbolo de Kotlin.              |
| `style`             | `"bold blue"`                        | El estilo del módulo.                                                   |
| `kotlin_binary`     | `"kotlin"`                           | Configura el binario kotlin que Starship ejecuta al obtener la versión. |
| `disabled`          | `false`                              | Deshabilita el módulo `kotlin`.                                         |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v1.4.21` | La versión de `kotlin`                 |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[kotlin]
symbol = "🅺 "
```

```toml
# ~/.config/starship.toml

[kotlin]
# Utiliza el binario del Compilador de Kotlin para obtener la versión instalada
kotlin_binary = "kotlinc"
```

## Kubernetes

Muestra el nombre del contexto actual de Kubernetes y, si se establece, el espacio de nombres del archivo kubeconfig. El espacio de nombres necesita establecerse en el archivo kubeconfig, esto puede hacerse mediante `kubectl config set-context starship-cluster --namespace astronaut`. Si se establece la variable de entorno `$KUBECONFIG`, el módulo usará eso si no usará el `~/.kube/config`.

::: consejo

Este módulo está deshabilitado por defecto. Para habilitarlo, establece `disabled` a `false` en tu archivo de configuración.

:::

### Opciones

| Opción            | Por defecto                                          | Descripción                                                                 |
| ----------------- | ---------------------------------------------------- | --------------------------------------------------------------------------- |
| `symbol`          | `"☸ "`                                               | Una cadena de formato que representa el símbolo mostrado antes del Cluster. |
| `format`          | `'[$symbol$context( \($namespace\))]($style) in '` | El formato del módulo.                                                      |
| `style`           | `"cyan bold"`                                        | El estilo del módulo.                                                       |
| `context_aliases` |                                                      | Tabla de alias de contexto a mostrar.                                       |
| `disabled`        | `true`                                               | Desactiva el módulo `kubernetes`.                                           |

### Variables

| Variable  | Ejemplo              | Descripción                                                 |
| --------- | -------------------- | ----------------------------------------------------------- |
| context   | `starship-cluster`   | El contexto actual de kubernetes                            |
| namespace | `starship-namespace` | Si se establece, el espacio de nombres actual de kubernetes |
| symbol    |                      | Refleja el valor de la opción `symbol`                      |
| style\* |                      | Refleja el valor de la opción `style`                       |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[kubernetes]
format = 'on [⛵ $context \($namespace\)](dimmed green) '
disabled = false
[kubernetes.context_aliases]
"dev.local.cluster.k8s" = "dev"
```

## Salto de línea

El módulo `line_break` separa el prompt en dos líneas.

### Opciones

| Opción     | Por defecto | Descripción                                                                    |
| ---------- | ----------- | ------------------------------------------------------------------------------ |
| `disabled` | `false`     | Deshabilita el módulo `line_break`, haciendo que el prompt sea una sola línea. |

### Ejemplo

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## Lua

El módulo `lua` muestra la versión instalada de Lua. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `.lua-version`
- El directorio actual contiene un directorio `lua`
- El directorio actual contiene un archivo con la extensión `.lua`

### Opciones

| Opción              | Por defecto                          | Descripción                                                          |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                               |
| `symbol`            | `"🌙 "`                               | Una cadena de formato que representa el símbolo de Lua.              |
| `detect_extensions` | `["lua"]`                            | Qué extensiones deberían activar este módulo.                        |
| `detect_files`      | `[".lua-version"]`                   | Qué nombres de archivo deberían activar este módulo.                 |
| `detect_folders`    | `["lua"]`                            | Qué carpetas deberían activar este módulo.                           |
| `style`             | `"bold blue"`                        | El estilo del módulo.                                                |
| `lua_binary`        | `"lua"`                              | Configura el binario lua que Starship ejecuta al obtener la versión. |
| `disabled`          | `false`                              | Deshabilita el módulo `lua`.                                         |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v5.4.0` | La versión de `lua`                    |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[lua]
format = "via [🌕 $version](bold blue) "
```

## Memoria utilizada

El módulo `memory_usage` muestra la memoria del sistema actual y el uso de la memoria de intercambio.

Por defecto, el uso de la memoria de intercambio se muestra si no es cero.

::: consejo

Este módulo está deshabilitado por defecto. Para habilitarlo, establece `disabled` a `false` en tu archivo de configuración.

:::

### Opciones

| Opción      | Por defecto                                   | Descripción                                                   |
| ----------- | --------------------------------------------- | ------------------------------------------------------------- |
| `threshold` | `75`                                          | Ocultar el uso de memoria a menos que supere este porcentaje. |
| `format`    | `"via $symbol [${ram}( | ${swap})]($style) "` | El formato del módulo.                                        |
| `symbol`    | `"🐏"`                                         | El símbolo usado antes de mostrar el uso de memoria.          |
| `style`     | `"bold dimmed white"`                         | El estilo del módulo.                                         |
| `disabled`  | `true`                                        | Desactiva el módulo `memory_usage`.                           |

### Variables

| Variable         | Ejemplo       | Descripción                                                                        |
| ---------------- | ------------- | ---------------------------------------------------------------------------------- |
| ram              | `31GiB/65GiB` | La memoria RAM usada/total del sistema actual.                                     |
| ram_pct          | `48%`         | El porcentaje de la memoria actual del sistema.                                    |
| swap\*\*     | `1GiB/4GiB`   | El tamaño de la memoria de intercambio del archivo de memoria del sistema actual.  |
| swap_pct\*\* | `77%`         | El porcentaje de memoria de intercambio del archivo de memoria del sistema actual. |
| symbol           | `🐏`           | Refleja el valor de la opción `symbol`                                             |
| style\*        |               | Refleja el valor de la opción `style`                                              |

\*: Esta variable sólo puede utilizarse como parte de una cadena de estilo \*\*: La información del archivo SWAP sólo se muestra si se detecta en el sistema actual

### Ejemplo

```toml
# ~/.config/starship.toml

[memory_usage]
disabled = false
threshold = -1
symbol = " "
style = "bold dimmed green"
```

## Mercurial Branch

El módulo `hg_branch` muestra la rama activa del repositorio en su directorio actual.

### Opciones

| Opción              | Por defecto                      | Descripción                                                                                         |
| ------------------- | -------------------------------- | --------------------------------------------------------------------------------------------------- |
| `symbol`            | `" "`                           | El símbolo usado antes del marcador hg o nombre de la rama del repositorio en su directorio actual. |
| `style`             | `"bold purple"`                  | El estilo del módulo.                                                                               |
| `format`            | `"on [$symbol$branch]($style) "` | El formato del módulo.                                                                              |
| `truncation_length` | `2^63 - 1`                       | Trunca el nombre de la rama hg a `N` grafemas                                                       |
| `truncation_symbol` | `"…"`                            | El símbolo usado para indicar que un nombre de rama fue truncado.                                   |
| `disabled`          | `true`                           | Desactiva el módulo `hg_branch`.                                                                    |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| branch    | `master` | La rama de mercurial activa            |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[hg_branch]
format = "on [🌱 $branch](bold purple)"
truncation_length = 4
truncation_symbol = ""
```

## Nim

El módulo `nim` muestra la versión instalada de Nim. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `nim.cfg`
- El directorio actual contiene un archivo con la extensión `.nim`
- El directorio actual contiene un archivo con la extensión `.nims`
- El directorio actual contiene un archivo con la extensión `.nimble`

### Opciones

| Opción              | Por defecto                          | Descripción                                          |
| ------------------- | ------------------------------------ | ---------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo                                |
| `symbol`            | `"👑 "`                               | El símbolo usado antes de mostrar la versión de Nim. |
| `detect_extensions` | `["nim", "nims", "nimble"]`          | Qué extensiones deberían activar este módulo.        |
| `detect_files`      | `["nim.cfg"]`                        | Qué nombres de archivo deberían activar este módulo. |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.           |
| `style`             | `"bold yellow"`                      | El estilo del módulo.                                |
| `disabled`          | `false`                              | Deshabilita el módulo `nim`.                         |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v1.2.0` | La versión de `nimc`                   |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[nim]
style = "yellow"
symbol = "🎣 "
```

## Nix-shell

El módulo `nix_shell` muestra el entorno nix-shell. El módulo se mostrará dentro de un entorno nix-shell.

### Opciones

| Opción       | Por defecto                                    | Descripción                                                                      |
| ------------ | ---------------------------------------------- | -------------------------------------------------------------------------------- |
| `format`     | `'via [$symbol$state( \($name\))]($style) '` | El formato del módulo.                                                           |
| `symbol`     | `"❄️ "`                                        | Una cadena de formato que representa el símbolo de nix-shell.                    |
| `style`      | `"bold blue"`                                  | El estilo del módulo.                                                            |
| `impure_msg` | `"impure"`                                     | Una cadena de formato que se muestra cuando el intérprete de comandos es impuro. |
| `pure_msg`   | `"pure"`                                       | Una cadena de formato que se muestra cuando el intérprete de comandos es puro.   |
| `disabled`   | `false`                                        | Desactiva el módulo `nix_shell`.                                                 |

### Variables

| Variable  | Ejemplo | Descripción                            |
| --------- | ------- | -------------------------------------- |
| state     | `pure`  | El estado de nix-shell                 |
| name      | `lorri` | El nombre de nix-shell                 |
| symbol    |         | Refleja el valor de la opción `symbol` |
| style\* |         | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[nix_shell]
disabled = true
impure_msg = "[impure shell](bold red)"
pure_msg = "[pure shell](bold green)"
format = 'via [☃️ $state( \($name\))](bold blue) '
```

## NodeJS

El módulo `nodejs` muestra la versión instalada de NodeJS. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `package.json`
- El directorio actual contiene un archivo `.node-version`
- El directorio actual contiene un directorio `node_modules`
- El directorio actual contiene un archivo con la extensión `.js`, `.mjs` o `.cjs`
- El directorio actual contiene un archivo con la extensión `.ts`

### Opciones

| Opción              | Por defecto                          | Descripción                                                                                                      |
| ------------------- | ------------------------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                                                                           |
| `symbol`            | `" "`                               | Una cadena de formato que representa el símbolo de NodeJS.                                                       |
| `detect_extensions` | `["js", "mjs", "cjs", "ts"]`         | Qué extensiones deberían activar este módulo.                                                                    |
| `detect_files`      | `["package.json", ".node-version"]`  | Qué nombres de archivo deberían activar este módulo.                                                             |
| `detect_folders`    | `["node_modules"]`                   | Qué carpetas deberían activar este módulo.                                                                       |
| `style`             | `"bold green"`                       | El estilo del módulo.                                                                                            |
| `disabled`          | `false`                              | Deshabilita el módulo `nodejs`.                                                                                  |
| `not_capable_style` | `bold red`                           | El estilo para el módulo cuando una propiedad de motores en `package.json` no coincide con la versión de NodeJS. |

### Variables

| Variable  | Ejemplo    | Descripción                            |
| --------- | ---------- | -------------------------------------- |
| version   | `v13.12.0` | La versión de `node`                   |
| symbol    |            | Refleja el valor de la opción `symbol` |
| style\* |            | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[nodejs]
format = "via [🤖 $version](bold green) "
```

## OCaml

El módulo `ocaml` muestra la versión actualmente instalada de OCaml. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo con extensión `.opam` o directorio `_opam`
- El directorio actual contiene un directorio `esy.lock`
- El directorio actual contiene un archivo `dune` o `dune-project`
- El directorio actual contiene un archivo `jbuild` o `jbuild-ignore`
- El directorio actual contiene un archivo `.merlin`
- El directorio actual contiene un archivo con la extensión `.ml`, `.mli`, `.re` o `.rei`

### Opciones

| Opción                    | Por defecto                                                                | Descripción                                                                |
| ------------------------- | -------------------------------------------------------------------------- | -------------------------------------------------------------------------- |
| `format`                  | `"via [$symbol($version )(\($switch_indicator$switch_name\) )]($style)"` | La cadena de formato para el módulo.                                       |
| `symbol`                  | `"🐫 "`                                                                     | El símbolo usado antes de mostrar la versión de OCaml.                     |
| `global_switch_indicator` | `""`                                                                       | La cadena de formato usada para representar el interruptor global de OPAM. |
| `local_switch_indicator`  | `"*"`                                                                      | La cadena de formato usada para representar el interruptor local de OPAM.  |
| `detect_extensions`       | `["opam", "ml", "mli", "re", "rei"]`                                       | Qué extensiones deberían activar este módulo.                              |
| `detect_files`            | `["dune", "dune-project", "jbuild", "jbuild-ignore", ".merlin"]`           | Qué nombres de archivo deberían activar este módulo.                       |
| `detect_folders`          | `["_opam", "esy.lock"]`                                                    | Qué carpetas deberían activar este módulo.                                 |
| `style`                   | `"bold yellow"`                                                            | El estilo del módulo.                                                      |
| `disabled`                | `false`                                                                    | Deshabilita el módulo `ocaml`.                                             |

### Variables

| Variable         | Ejemplo      | Descripción                                                                 |
| ---------------- | ------------ | --------------------------------------------------------------------------- |
| version          | `v4.10.0`    | La versión de `ocaml`                                                       |
| switch_name      | `my-project` | El interruptor OPAM activo                                                  |
| switch_indicator |              | Refleja el valor de `indicator` para el interruptor OPAM activo actualmente |
| symbol           |              | Refleja el valor de la opción `symbol`                                      |
| style\*        |              | Refleja el valor de la opción `style`                                       |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[ocaml]
format = "via [🐪 $version]($style) "
```

## OpenStack

El módulo `openstack` muestra la nube actual y el proyecto OpenStack. El módulo solo está activo cuando la variable de entorno `OS_CLOUD` está definida, en cuyo caso leerá el archivo `clouds.yaml` desde cualquiera de las [ubicaciones por defecto](https://docs.openstack.org/python-openstackclient/latest/configuration/index.html#configuration-files). para obtener el proyecto actual en uso.

### Opciones

| Opción     | Por defecto                                         | Descripción                                                 |
| ---------- | --------------------------------------------------- | ----------------------------------------------------------- |
| `format`   | `"on [$symbol$cloud(\\($project\\))]($style) "` | El formato del módulo.                                      |
| `symbol`   | `"☁️ "`                                             | El símbolo usado antes de mostrar la nube OpenStack actual. |
| `style`    | `"bold yellow"`                                     | El estilo del módulo.                                       |
| `disabled` | `false`                                             | Deshabilita el módulo `openstack`.                          |

### Variables

| Variable  | Ejemplo | Descripción                            |
| --------- | ------- | -------------------------------------- |
| cloud     | `corp`  | La nube OpenStack actual               |
| project   | `dev`   | El proyecto OpenStack actual           |
| symbol    |         | Refleja el valor de la opción `symbol` |
| style\* |         | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[openstack]
format = "on [$symbol$cloud(\\($project\\))]($style) "
style = "bold yellow"
symbol = "☁️ "
```

## Versión del paquete

El módulo `package` se muestra cuando el directorio actual es el repositorio de un paquete, y muestra su versión actual. El módulo soporta actualmente los paquetes `npm`, `cargo`, `poetry`, `composer`, `gradle`, `julia`, `mix` y `helm`.

- **npm** – La versión del paquete `npm` se extrae del `package.json` presente en el directorio actual
- **cargo** – La versión del paquete `cargo` se extrae del `Cargo.toml` presente en el directorio actual
- **poetry** – La versión del paquete `poetry` es extraída del `pyproject.toml` presente en el directorio actual
- **composer** - La versión del paquete `composer` se extrae del `composer.json` presente en el directorio actual
- **gradle** – La versión del paquete `gradle` es extraída del `build.gradle` presente
- **julia** - La versión del paquete se extrae del `Project.toml` presente
- **mix** - La versión del paquete `mix` se extrae del `mix.exs` presente
- **helm** - La versión del gráfico `helm` se extrae del `Chart.yaml` presente
- **maven** - La versión del paquete `maven` es extraída del `pom.xml` presente
- **meson** - La versión del paquete `meson` es extraída del `meson.build` presente

> ⚠️ La versión que se muestra es la del paquete cuyo código fuente está en tu directorio actual, no en tu gestor de paquetes.

### Opciones

| Opción            | Por defecto                       | Descripción                                                                      |
| ----------------- | --------------------------------- | -------------------------------------------------------------------------------- |
| `format`          | `"is [$symbol$version]($style) "` | El formato del módulo.                                                           |
| `symbol`          | `"📦 "`                            | El símbolo usado antes de mostrar la versión del paquete.                        |
| `style`           | `"bold 208"`                      | El estilo del módulo.                                                            |
| `display_private` | `false`                           | Activar la visualización de la versión para los paquetes marcados como privados. |
| `disabled`        | `false`                           | Desactiva el módulo `package`.                                                   |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v1.0.0` | La versión de su paquete               |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[package]
format = "via [🎁 $version](208 bold) "
```

## Perl

El módulo `perl` muestra la versión actualmente instalada de Perl. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `Makefile.PL` o `Build.PL`
- El directorio actual contiene un archivo `cpanfile` o `cpanfile.snapshot`
- El directorio actual contiene un archivo `META.json` o `META.yml`
- El directorio actual contiene un archivo `.perl-version`
- El directorio actual contiene un `.pl`, `.pm` o `.pod`

### Opciones

| Opción              | Por defecto                                                                                              | Descripción                                          |
| ------------------- | -------------------------------------------------------------------------------------------------------- | ---------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`                                                                     | La cadena de formato para el módulo.                 |
| `symbol`            | `"🐪 "`                                                                                                   | El símbolo usado antes de mostrar la versión de Perl |
| `detect_extensions` | `["pl", "pm", "pod"]`                                                                                    | Qué extensiones deberían activar este módulo.        |
| `detect_files`      | `["Makefile.PL", "Build.PL", "cpanfile", "cpanfile.snapshot", "META.json", "META.yml", ".perl-version"]` | Qué nombres de archivo deberían activar este módulo. |
| `detect_folders`    | `[]`                                                                                                     | Qué carpetas deberían activar este módulo.           |
| `style`             | `"bold 149"`                                                                                             | El estilo del módulo.                                |
| `disabled`          | `false`                                                                                                  | Deshabilita el módulo `perl`.                        |

### Variables

| Variable  | Ejemplo   | Descripción                            |
| --------- | --------- | -------------------------------------- |
| version   | `v5.26.1` | La versión de `perl`                   |
| symbol    |           | Refleja el valor de la opción `symbol` |
| style\* |           | Refleja el valor de la opción `style`  |

### Ejemplo

```toml
# ~/.config/starship.toml

[perl]
format = "via [🦪 $version]($style) "
```

## PHP

El módulo `php` muestra la versión instalada de PHP. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `composer.json`
- El directorio actual contiene un archivo `.php-version`
- El directorio actual contiene una extensión `.php`

### Opciones

| Opción              | Por defecto                          | Descripción                                          |
| ------------------- | ------------------------------------ | ---------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                               |
| `symbol`            | `"🐘 "`                               | El símbolo usado antes de mostrar la versión de PHP. |
| `detect_extensions` | `["php"]`                            | Qué extensiones deberían activar este módulo.        |
| `detect_files`      | `["composer.json", ".php-version"]`  | Qué nombres de archivo deberían activar este módulo. |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.           |
| `style`             | `"147 bold"`                         | El estilo del módulo.                                |
| `disabled`          | `false`                              | Deshabilita el módulo `php`.                         |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v7.3.8` | La versión de `php`                    |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[php]
format = "via [🔹 $version](147 bold) "
```

## PureScript

El módulo `purescript` muestra la versión actualmente instalada de PureScript. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `spago.dhall`
- El directorio actual contiene un archivo con la extensión `.purs`

### Opciones

| Opción              | Por defecto                          | Descripción                                                 |
| ------------------- | ------------------------------------ | ----------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                      |
| `symbol`            | `"<=> "`                       | El símbolo usado antes de mostrar la versión de PureScript. |
| `detect_extensions` | `["purs"]`                           | Qué extensiones deberían activar este módulo.               |
| `detect_files`      | `["spago.dhall"]`                    | Qué nombres de archivo deberían activar este módulo.        |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                  |
| `style`             | `"bold white"`                       | El estilo del módulo.                                       |
| `disabled`          | `false`                              | Deshabilita el módulo `purescript`.                         |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `0.13.5` | La versión de `purescript`             |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[purescript]
format = "via [$symbol$version](bold white)"
```

## Python

El módulo `python` muestra la versión actualmente instalada de Python y el actual entorno virtual de Python si uno está activado.

Si `pyenv_version_name` se establece en `true`, mostrará el nombre de la versión de pyenv. De lo contrario, se mostrará el número de versión de `python --version`.

Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `.python-version`
- El directorio actual contiene un archivo `Pipfile`
- El directorio actual contiene un archivo `__init__.py`
- El directorio actual contiene un archivo `pyproject.toml`
- El directorio actual contiene un archivo `requirements.txt`
- El directorio actual contiene un archivo `setup.py`
- El directorio actual contiene un archivo `tox.ini`
- El directorio actual contiene un archivo con la extensión `.py`.
- Un entorno virtual está activado actualmente

### Opciones

| Opción               | Por defecto                                                                                                  | Descripción                                                                           |
| -------------------- | ------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------- |
| `format`             | `'via [${symbol}${pyenv_prefix}(${version} )(\($virtualenv\) )]($style)'`                                  | El formato del módulo.                                                                |
| `symbol`             | `"🐍 "`                                                                                                       | Una cadena de formato que representa el símbolo de Python                             |
| `style`              | `"yellow bold"`                                                                                              | El estilo del módulo.                                                                 |
| `pyenv_version_name` | `false`                                                                                                      | Usar pyenv para obtener la versión de Python                                          |
| `pyenv_prefix`       | `pyenv`                                                                                                      | Prefijo antes de mostrar la versión de pyenv sólo se utiliza si se utiliza pyenv      |
| `python_binary`      | `["python", "python3, "python2"]`                                                                            | Configura los binarios de python que Starship debería ejecutar al obtener la versión. |
| `detect_extensions`  | `[".py"]`                                                                                                    | Qué extensiones deben activar este módulo                                             |
| `detect_files`       | `[".python-version", "Pipfile", "__init__.py", "pyproject.toml", "requirements.txt", "setup.py", "tox.ini"]` | Qué nombres de archivo deben activar este módulo                                      |
| `detect_folders`     | `[]`                                                                                                         | Qué carpetas deben activar este módulo                                                |
| `disabled`           | `false`                                                                                                      | Deshabilita el módulo `python`.                                                       |

::: consejo

La variable `python_binary` acepta una cadena o una lista de cadenas. Starship intentará ejecutar cada binario hasta que obtenga un resultado. Ten en cuenta que sólo puedes cambiar el binario que Starship ejecuta para obtener la versión de Python no los argumentos que se utilizan.

Los valores por defecto y el orden para `python_binary` fue elegido para identificar primero la versión de Python en un entorno virtualenv/conda (que actualmente añade un `python`, no importa si apunta a `pithon3` o `pithon2`). Esto tiene el efecto secundario que si todavía tienes un sistema de Python 2 instalado, puede ser recogido antes de cualquier Python 3 (al menos en las Distros de Linux que siempre enlazan `/usr/bin/python` a Python 2). Si ya no trabajas con Python 2 pero no puedes removerlo del sistema, cambiando esto a `"python3"` ocultará cualquier versión de Python 2, ver ejemplo a continuación.

:::

### Variables

| Variable     | Ejemplo         | Descripción                                 |
| ------------ | --------------- | ------------------------------------------- |
| version      | `"v3.8.1"`      | La versión de `python`                      |
| symbol       | `"🐍 "`          | Refleja el valor de la opción `symbol`      |
| style        | `"yellow bold"` | Refleja el valor de la opción `style`       |
| pyenv_prefix | `"pyenv "`      | Ordena el valor de la opción `pyenv_prefix` |
| virtualenv   | `"venv"`        | El nombre actual del `virtualenv`           |

### Ejemplo

```toml
# ~/.config/starship.toml

[python]
symbol = "👾 "
pyenv_version_name = true
```

```toml
# ~/.config/starship.toml

[python]
# Solo usa el binario `python3` para obtener la versión.
python_binary = "python3"
```

```toml
# ~/.config/starship.toml

[python]
# No se dispara con archivos con extensión py
detect_extensions = []
```

## Ruby

Por defecto, el módulo `ruby` muestra la versión actualmente instalada de Ruby. El módulo se mostrará si se cumplen alguna de las siguientes condiciones:

- El directorio actual contiene un archivo `Gemfile`
- El directorio actual contiene un archivo `.ruby-version`
- El directorio actual contiene un archivo `.rb`

### Opciones

| Opción              | Por defecto                          | Descripción                                              |
| ------------------- | ------------------------------------ | -------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                   |
| `symbol`            | `"💎 "`                               | Una cadena de formato que representa el símbolo de Ruby. |
| `detect_extensions` | `["rb"]`                             | Qué extensiones deberían activar este módulo.            |
| `detect_files`      | `["Gemfile", ".ruby-version"]`       | Qué nombres de archivo deberían activar este módulo.     |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.               |
| `style`             | `"bold red"`                         | El estilo del módulo.                                    |
| `disabled`          | `false`                              | Deshabilita el módulo `ruby`.                            |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v2.5.1` | La versión de `ruby`                   |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[ruby]
symbol = "🔺 "
```

## Rust

Por defecto, el módulo `rust` muestra la versión actualmente instalada de Rust. El módulo se mostrará si se cumplen alguna de las siguientes condiciones:

- El directorio actual contiene un archivo `Cargo.toml`
- El directorio actual contiene un archivo con la extensión `.rs`

### Opciones

| Opción              | Por defecto                          | Descripción                                             |
| ------------------- | ------------------------------------ | ------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                  |
| `symbol`            | `"🦀 "`                               | Una cadena de formato que representa el símbolo de Rust |
| `detect_extensions` | `["rs"]`                             | Qué extensiones deberían activar este módulo.           |
| `detect_files`      | `["Cargo.toml"]`                     | Qué nombres de archivo deberían activar este módulo.    |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.              |
| `style`             | `"bold red"`                         | El estilo del módulo.                                   |
| `disabled`          | `false`                              | Deshabilita el módulo `rust`.                           |

### Variables

| Variable  | Ejemplo           | Descripción                            |
| --------- | ----------------- | -------------------------------------- |
| version   | `v1.43.0-nightly` | La versión de `rustc`                  |
| symbol    |                   | Refleja el valor de la opción `symbol` |
| style\* |                   | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[rust]
format = "via [⚙️ $version](red bold)"
```

## Scala

El módulo `scala` muestra la versión actualmente instalada de Scala. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `build.sbt`, `.scalaenv` o `.sbtenv`
- El directorio actual contiene un archivo con la extensión `.scala` o `.sbt`
- El directorio actual contiene un directorio llamado `.metals`

### Opciones

| Opción              | Por defecto                              | Descripción                                               |
| ------------------- | ---------------------------------------- | --------------------------------------------------------- |
| `format`            | `"via [${symbol}(${version} )]($style)"` | El formato del módulo.                                    |
| `detect_extensions` | `["sbt", "scala"]`                       | Qué extensiones deberían activar este módulo.             |
| `detect_files`      | `[".scalaenv", ".sbtenv", "build.sbt"]`  | Qué nombres de archivo deberían activar este módulo.      |
| `detect_folders`    | `[".metals"]`                            | Qué carpetas deberían activar estos módulos.              |
| `symbol`            | `"🆂 "`                                   | Una cadena de formato que representa el símbolo de Scala. |
| `style`             | `"red dimmed"`                           | El estilo del módulo.                                     |
| `disabled`          | `false`                                  | Deshabilita el módulo `scala`.                            |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `2.13.5` | La versión de `scala`                  |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[scala]
symbol = "🌟 "
```

## Shell

El módulo `shell` muestra un indicador para el intérprete de comandos actualmente utilizado.

::: consejo

Este módulo está deshabilitado por defecto. Para habilitarlo, establece `disabled` a `false` en tu archivo de configuración.

:::

### Opciones

| Opción                 | Por defecto  | Descripción                                              |
| ---------------------- | ------------ | -------------------------------------------------------- |
| `bash_indicator`       | `bsh`        | Una cadena de formato usada para representar Bash.       |
| `fish_indicator`       | `fsh`        | Una cadena de formato usada para representar Fish.       |
| `zsh_indicator`        | `zsh`        | Una cadena de formato usada para representar Zsh.        |
| `powershell_indicator` | `psh`        | Una cadena de formato usada para representar Powershell. |
| `ion_indicator`        | `ion`        | Una cadena de formato usada para representar Ion.        |
| `elvish_indicator`     | `esh`        | Una cadena de formato usada para representar Elvish.     |
| `tcsh_indicator`       | `tsh`        | A format string used to represent tcsh.                  |
| `format`               | `$indicator` | El formato del módulo.                                   |
| `disabled`             | `true`       | Deshabilita el módulo `shell`.                           |

### Variables

| Variable  | Por defecto | Descripción                                                                          |
| --------- | ----------- | ------------------------------------------------------------------------------------ |
| indicator |             | Ordena el valor de `indicator` para el intérprete de comandos actualmente utilizado. |

### Ejemplos

```toml
# ~/.config/starship.toml

[shell]
fish_indicator = ""
powershell_indicator = "_"
disabled = false
```

## SHLVL

El módulo `shlvl` muestra la variable de entorno actual `SHLVL` ("nivel de shell"), si está establecido en un número y alcanza o supera el umbral especificado.

### Opciones

| Opción      | Por defecto                  | Descripción                                                       |
| ----------- | ---------------------------- | ----------------------------------------------------------------- |
| `threshold` | `2`                          | Mostrar el umbral.                                                |
| `format`    | `"[$symbol$shlvl]($style) "` | El formato del módulo.                                            |
| `symbol`    | `"↕️ "`                      | El símbolo utilizado para representar el `SHLVL`.                 |
| `repeat`    | `false`                      | Hace que el `symbol` se repita con la cantidad actual de `SHLVL`. |
| `style`     | `"bold yellow"`              | El estilo del módulo.                                             |
| `disabled`  | `true`                       | Deshabilita el módulo `shlvl`.                                    |

### Variables

| Variable  | Ejemplo | Descripción                            |
| --------- | ------- | -------------------------------------- |
| shlvl     | `3`     | El valor actual de `SHLVL`             |
| symbol    |         | Refleja el valor de la opción `symbol` |
| style\* |         | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[shlvl]
disabled = false
format = "$shlvl level(s) down"
threshold = 3
```

## Singularity

El módulo `singularity` muestra la imagen de singularity actual, si se encuentra dentro de un contenedor y `$SINGULARITY_NAME` está establecido.

### Opciones

| Opción     | Por defecto                      | Descripción                                                         |
| ---------- | -------------------------------- | ------------------------------------------------------------------- |
| `format`   | `'[$symbol\[$env\]]($style) '` | El formato del módulo.                                              |
| `symbol`   | `""`                             | Una cadena de formato que se muestra antes del nombre de la imagen. |
| `style`    | `"bold dimmed blue"`             | El estilo del módulo.                                               |
| `disabled` | `false`                          | Deshabilita el módulo `singularity`.                                |

### Variables

| Variable  | Ejemplo      | Descripción                            |
| --------- | ------------ | -------------------------------------- |
| env       | `centos.img` | La imagen de singularity actual        |
| symbol    |              | Refleja el valor de la opción `symbol` |
| style\* |              | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[singularity]
format = '[📦 \[$env\]]($style) '
```

## Status

El módulo `status` muestra el código de salida del comando anterior. El módulo se mostrará sólo si el código de salida no es `0`.

::: consejo

Este módulo está deshabilitado por defecto. Para habilitarlo, establece `disabled` a `false` en tu archivo de configuración.

:::

::: advertencia Este módulo no es compatible con el intérprete de comandos Elvish. :::

### Opciones

| Opción                  | Por defecto                   | Descripción                                            |
| ----------------------- | ----------------------------- | ------------------------------------------------------ |
| `format`                | `"[$symbol$status]($style) "` | El formato del módulo                                  |
| `symbol`                | `"✖"`                         | El símbolo mostrado en un error del programa           |
| `not_executable_symbol` | `"🚫"`                         | El símbolo mostrado cuando el archivo no es ejecutable |
| `not_found_symbol`      | `"🔍"`                         | El símbolo mostrado cuando no se encuentra el comando  |
| `sigint_symbol`         | `"🧱"`                         | El símbolo mostrado en SIGINT (Ctrl + c)               |
| `signal_symbol`         | `"⚡"`                         | El símbolo mostrado en cualquier señal                 |
| `style`                 | `"bold red"`                  | El estilo del módulo.                                  |
| `recognize_signal_code` | `true`                        | Habilitar mapeo de señales desde el código de salida   |
| `map_symbol`            | `false`                       | Habilitar mapeo de símbolos desde el código de salida  |
| `disabled`              | `true`                        | Deshabilita el módulo `status`.                        |

### Variables

| Variable       | Ejemplo | Descripción                                                                          |
| -------------- | ------- | ------------------------------------------------------------------------------------ |
| status         | `127`   | El código de salida del último comando                                               |
| int            | `127`   | El código de salida del último comando                                               |
| common_meaning | `ERROR` | Comprobación del código si no es una señal                                           |
| signal_number  | `9`     | Número de señal correspondiente al código de salida, sólo si está señalizada         |
| signal_name    | `KILL`  | Nombre de la señal correspondiente al código de salida, sólo si está señalizada      |
| maybe_int      | `7`     | Contiene el número de código de salida cuando no se ha encontrado ningún significado |
| symbol         |         | Refleja el valor de la opción `symbol`                                               |
| style\*      |         | Refleja el valor de la opción `style`                                                |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml

# ~/.config/starship.toml

[status]
style = "bg:blue"
symbol = "🔴"
format = '[\[$symbol $common_meaning$signal_name$maybe_int\]]($style) '
map_symbol = true
disabled = false

```

## Swift

Por defecto, el módulo `swift` muestra la versión actualmente instalada de Swift. El módulo se mostrará si se cumplen alguna de las siguientes condiciones:

- El directorio actual contiene un archivo `Package.swift`
- El directorio actual contiene un archivo con la extensión `.swift`

### Opciones

| Opción              | Por defecto                          | Descripción                                              |
| ------------------- | ------------------------------------ | -------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                   |
| `symbol`            | `"🐦 "`                               | Una cadena de formato que representa el símbolo de Swift |
| `detect_extensions` | `["swift"]`                          | Qué extensiones deberían activar este módulo.            |
| `detect_files`      | `["Package.swift"]`                  | Qué nombres de archivo deberían activar este módulo.     |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.               |
| `style`             | `"bold 202"`                         | El estilo del módulo.                                    |
| `disabled`          | `false`                              | Deshabilita el módulo `swift`.                           |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v5.2.4` | La versión de `swift`                  |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[swift]
format = "via [🏎  $version](red bold)"
```

## Terraform

El módulo `terraform` muestra el espacio de trabajo y la versión actual de Terraform.

::: consejo

Por defecto la versión de Terraform no se muestra, ya que esto es lento en las versiones actuales de Terraform cuando muchos plugins están en uso. Si aún deseas activarlo, [sigue el ejemplo que se muestra a continuación](#with-version).

:::

Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene una carpeta `.terraform`
- El directorio actual contiene un archivo con las extensiones `.tf` o `.hcl`

### Opciones

| Opción              | Por defecto                          | Descripción                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol$workspace]($style) "` | La cadena de formato para el módulo.                                            |
| `symbol`            | `"💠"`                                | Una cadena de formato que se muestra antes del espacio de trabajo de Terraform. |
| `detect_extensions` | `["tf", "hcl"]`                      | Qué extensiones deberían activar este módulo.                                   |
| `detect_files`      | `[]`                                 | Qué nombres de archivo deberían activar este módulo.                            |
| `detect_folders`    | `[".terraform"]`                     | Qué carpetas deberían activar este módulo.                                      |
| `style`             | `"bold 105"`                         | El estilo del módulo.                                                           |
| `disabled`          | `false`                              | Desactiva el módulo `terraform`.                                                |

### Variables

| Variable  | Ejemplo       | Descripción                               |
| --------- | ------------- | ----------------------------------------- |
| version   | `v0.12.24`    | La versión de `terraform`                 |
| workspace | `por defecto` | El espacio de trabajo actual de Terraform |
| symbol    |               | Refleja el valor de la opción `symbol`    |
| style\* |               | Refleja el valor de la opción `style`     |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

#### Con la versión

```toml
# ~/.config/starship.toml

[terraform]
format = "[🏎💨 $version$workspace]($style) "
```

#### Sin la versión

```toml
# ~/.config/starship.toml

[terraform]
format = "[🏎💨 $workspace]($style) "
```

## Hora

El módulo `time` muestra la hora **local** actual. El valor de configuración de `format` es usado por la caja de [`chrono`](https://crates.io/crates/chrono) para controlar cómo se muestra la hora. Echa un vistazo a [los documentos de chrono strftime](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) para ver qué opciones están disponibles.

::: consejo

Este módulo está deshabilitado por defecto. Para habilitarlo, establece `disabled` a `false` en tu archivo de configuración.

:::

### Opciones

| Opción            | Por defecto             | Descripción                                                                                                                                                                 |
| ----------------- | ----------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | `"at [$time]($style) "` | La cadena de formato para el módulo.                                                                                                                                        |
| `use_12hr`        | `false`                 | Habilita el formato de 12 horas                                                                                                                                             |
| `time_format`     | ver abajo               | La [cadena de formato de chrono](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) utilizada para formatear la hora.                                          |
| `style`           | `"bold yellow"`         | El estilo del módulo de la hora                                                                                                                                             |
| `utc_time_offset` | `"local"`               | Establece el desplazamiento UTC a utilizar. Rango de -24 &lt; x &lt; 24. Permite a los flotantes acomodar los desplazamientos de zona horaria de 30/45 minutos. |
| `disabled`        | `true`                  | Deshabilita el módulo `time`.                                                                                                                                               |
| `time_range`      | `"-"`                   | Establece el intervalo de tiempo durante el cual el módulo se mostrará. Las horas deben especificarse en formato de 24 horas                                                |

Si `use_12hr` es `true`, entonces `time_format` por defecto es `"%r"`. De lo contrario, el valor por defecto es `"%T"`. Configurar manualmente `time_format` sobrescribirá la configuración `use_12hr`.

### Variables

| Variable  | Ejemplo    | Descripción                           |
| --------- | ---------- | ------------------------------------- |
| time      | `13:08:10` | La hora actual.                       |
| style\* |            | Refleja el valor de la opción `style` |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = '🕙[\[ $time \]]($style) '
time_format = "%T"
utc_time_offset = "-5"
time_range = "10:00:00-14:00:00"
```

## Username

El módulo `username` muestra el nombre de usuario activo. El módulo se mostrará si se cumplen alguna de las siguientes condiciones:

- El usuario actual es root
- El usuario actual no es el mismo que el que está conectado
- El usuario está actualmente conectado como una sesión SSH
- La variable `show_always` se establece en true

::: consejo

La conexión SSH se detecta comprobando las variables de entorno `SSH_CONNECTION`, `SSH_CLIENT`, y `SSH_TTY`. Si tu host SSH no configura estas variables, una solución es establecer una de ellas con un valor tonto.

:::

### Opciones

| Opción        | Por defecto             | Descripción                                |
| ------------- | ----------------------- | ------------------------------------------ |
| `style_root`  | `"bold red"`            | El estilo usado cuando el usuario es root. |
| `style_user`  | `"bold yellow"`         | El estilo usado para usuarios no root.     |
| `format`      | `"[$user]($style) in "` | El formato del módulo.                     |
| `show_always` | `false`                 | Siempre muestra el módulo `username`.      |
| `disabled`    | `false`                 | Deshabilita el módulo `username`.          |

### Variables

| Variable | Ejemplo      | Descripción                                                                                         |
| -------- | ------------ | --------------------------------------------------------------------------------------------------- |
| `style`  | `"red bold"` | Refleja el valor de la opción `style_root` cuando root inició sesión y `style_user` por otra parte. |
| `user`   | `"matchai"`  | El ID de usuario conectado actualmente.                                                             |

### Ejemplo

```toml
# ~/.config/starship.toml

[username]
style_user = "white bold"
style_root = "black bold"
format = "user: [$user]($style) "
disabled = false
show_always = true
```

## Vagrant

El módulo `vagrant` muestra la versión instalada de Vagrant. Por defecto, el módulo se mostrará si se cumplen cualquiera de las siguientes condiciones:

- El directorio actual contiene un archivo `Vagrantfile`

### Opciones

| Opción              | Por defecto                          | Descripción                                                 |
| ------------------- | ------------------------------------ | ----------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                                      |
| `symbol`            | `"⍱ "`                               | Una cadena de formato que representa el símbolo de Vagrant. |
| `detect_extensions` | `[]`                                 | Qué extensiones deberían activar este módulo.               |
| `detect_files`      | `["Vagrantfile"]`                    | Qué nombres de archivo deberían activar este módulo.        |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.                  |
| `style`             | `"cyan bold"`                        | El estilo del módulo.                                       |
| `disabled`          | `false`                              | Deshabilita el módulo `vagrant`.                            |

### Variables

| Variable  | Ejemplo          | Descripción                            |
| --------- | ---------------- | -------------------------------------- |
| version   | `Vagrant 2.2.10` | La versión de `Vagrant`                |
| symbol    |                  | Refleja el valor de la opción `symbol` |
| style\* |                  | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[vagrant]
format = "via [⍱ $version](bold white) "
```

## VCSH

El módulo `vcsh` muestra el repositorio VCSH activo actual. El módulo sólo se mostrará si un repositorio está actualmente en uso.

### Opciones

| Opción     | Por defecto                      | Descripción                                                  |
| ---------- | -------------------------------- | ------------------------------------------------------------ |
| `symbol`   |                                  | El símbolo usado antes de mostrar el nombre del repositorio. |
| `style`    | `"bold yellow"`                  | El estilo del módulo.                                        |
| `format`   | `"vcsh [$symbol$repo]($style) "` | El formato del módulo.                                       |
| `disabled` | `false`                          | Deshabilita el módulo `vcsh`.                                |

### Variables

| Variable    | Ejemplo                                                     | Descripción                            |
| ----------- | ----------------------------------------------------------- | -------------------------------------- |
| repositorio | `dotfiles` si está en un repositorio VCSH nombrado dotfiles | El nombre del repositorio activo       |
| symbol      |                                                             | Refleja el valor de la opción `symbol` |
| style\*   | `black bold dimmed`                                         | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[vcsh]
format = "[🆅 $repo](bold blue) "
```

## Zig

Por defecto, el módulo `zig` muestra la versión instalada de Zig. El módulo se mostrará si se cumplen alguna de las siguientes condiciones:

- El directorio actual contiene un archivo `.zig`

### Opciones

| Opción              | Por defecto                          | Descripción                                          |
| ------------------- | ------------------------------------ | ---------------------------------------------------- |
| `symbol`            | `"↯ "`                               | El símbolo usado antes de mostrar la versión de Zig. |
| `style`             | `"bold yellow"`                      | El estilo del módulo.                                |
| `format`            | `"via [$symbol($version )]($style)"` | El formato del módulo.                               |
| `disabled`          | `false`                              | Deshabilita el módulo `zig`.                         |
| `detect_extensions` | `["zig"]`                            | Qué extensiones deberían activar este módulo.        |
| `detect_files`      | `[]`                                 | Qué nombres de archivo deberían activar este módulo. |
| `detect_folders`    | `[]`                                 | Qué carpetas deberían activar este módulo.           |

### Variables

| Variable  | Ejemplo  | Descripción                            |
| --------- | -------- | -------------------------------------- |
| version   | `v0.6.0` | La versión de `zig`                    |
| symbol    |          | Refleja el valor de la opción `symbol` |
| style\* |          | Refleja el valor de la opción `style`  |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

### Ejemplo

```toml
# ~/.config/starship.toml

[zig]
symbol = "⚡️ "
```

## Comandos personalizados

Los módulos `personalizados` muestran la salida de algunos comandos arbitrarios.

Estos módulos se mostrarán si se cumplen alguna de las siguientes condiciones:

- El directorio actual contiene un archivo cuyo nombre está en `files`
- El directorio actual contiene un directorio cuyo nombre está en `directories`
- El directorio actual contiene un archivo cuya extensión está en `extensions`
- El comando `when` devuelve 0

::: consejo

Múltiples módulos personalizados pueden definirse usando un `.` (punto).

:::

::: consejo

El orden en el que se muestran los módulos personalizados se puede establecer individualmente incluyendo `${custom.foo}` en el `format` de nivel superior (ya que incluye un punto, necesita usar `${...}`). Por defecto, el módulo `personalizado` simplemente mostrará todos los módulos personalizados en el orden en que fueron definidos.

:::

::: consejo

[El problema #1252](https://github.com/starship/starship/discussions/1252) contiene ejemplos de módulos personalizados. ¡Si tienes un ejemplo interesante no cubierto allí, no dudes en compartirlo!

:::

### Opciones

| Opción        | Por defecto                     | Descripción                                                                                                                                           |
| ------------- | ------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| `comando`     |                                 | El comando cuya salida debe ser impresa. El comando se pasará en stdin al intérprete de comandos.                                                     |
| `cuando`      |                                 | Comando del intérprete de comandos usado como condición para mostrar el módulo. El módulo se mostrará si el comando devuelve un código de estado `0`. |
| `shell`       |                                 | [Ver abajo](#custom-command-shell)                                                                                                                    |
| `descripción` | `"<custom module>"`       | La descripción del módulo que se muestra al ejecutar `starship explain`.                                                                              |
| `archivos`    | `[]`                            | Los archivos que se buscarán en el directorio de trabajo para obtener una coincidencia.                                                               |
| `directorios` | `[]`                            | Los directorios que se buscarán en el directorio de trabajo para una coincidencia.                                                                    |
| `extensiones` | `[]`                            | Las extensiones que se buscarán en el directorio de trabajo para obtener una coincidencia.                                                            |
| `symbol`      | `""`                            | El símbolo usado antes de mostrar la salida del comando.                                                                                              |
| `style`       | `"bold green"`                  | El estilo del módulo.                                                                                                                                 |
| `format`      | `"[$symbol($output )]($style)"` | El formato del módulo.                                                                                                                                |
| `disabled`    | `false`                         | Deshabilita este `módulo` personalizado.                                                                                                              |

### Variables

| Variable  | Descripción                                                 |
| --------- | ----------------------------------------------------------- |
| salida    | La salida del comando del intérprete de comandos en `shell` |
| symbol    | Refleja el valor de la opción `symbol`                      |
| style\* | Refleja el valor de la opción `style`                       |

\*: Esta variable sólo puede ser usada como parte de una cadena de estilo

#### Comando personalizado del intérprete de comandos

`shell` acepta una lista no vacía de cadenas, donde:

- La primera cadena es la ruta al intérprete de comandos a usar para ejecutar el comando.
- Otros argumentos siguientes que son pasados al shell.

Si no está activado, se retornará a STARSHIP_SHELL y luego a "sh" en Linux, y "cmd /C" en Windows.

El `comando` será pasado en stdin.

Si no se da el `shell` o solo contiene un elemento y Starship detecta PowerShell los siguientes argumentos se añadirán automáticamente: `-NoProfile -Command -`. Este comportamiento puede evitarse pasando explícitamente argumentos al intérprete de comandos, p.ej.

```toml
shell = ["pwsh", "-Command", "-"]
```

::: advertencia Asegúrate de que tu configuración personalizada del intérprete de comandos salga con éxito

Si estableces un comando personalizado, asegúrate de que el intérprete de comandos por defecto usado por Starship ejecutará correctamente el comando con una salida elegante (a través de la opción `shell`).

Por ejemplo, PowerShell requiere el parámetro `-Command` para ejecutar una sola línea. Omitir este parámetro puede arrojar a Starship a un bucle recursivo donde el intérprete de comandos podría intentar cargar un entorno de perfil completo con Starship en sí misma y volver a ejecutar el comando personalizado, entrando en un bucle infinito.

Se recomiendan parámetros similares a `-NoProfile` en PowerShell para otros intérprete de comandos también para evitar tiempo extra de carga de un perfil personalizado en cada invocación de Starship.

La detección automática de intérpretes de comandos y la adición adecuada de parámetros están actualmente implementados, pero es posible que no todos los intérpretes de comandos estén cubiertos. Por favor, [abre un problema](https://github.com/starship/starship/issues/new/choose) con los detalles del intérprete de comandos y la configuración de Starship si te encuentras en tal escenario.

:::

### Ejemplo

```toml
# ~/.config/starship.toml

[custom.foo]
command = "echo foo"  # mostrar resultado de command
files = ["foo"]       # can specify filters
when = """ test "$HOME" == "$PWD" """
format = " transcending [$output]($style)"

[custom.time]
command = "time /T"
files = ["*.pst"]
shell = ["pwsh.exe", "-NoProfile", "-Command", "-"]
```
