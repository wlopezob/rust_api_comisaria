# API COMISARIA

![RUST](https://raw.githubusercontent.com/wlopezob/rust_api_comisaria/main/api.png "RUST")

## Crates:
- tokio
- axum
- serde
- reqwest
- mongodb
- dotenvy
- async-trait

## Folders:
- utils: Metodos utilitarios
- services: Solicita los datos del repository o api_caller
- routes: Creacion del router http
- repository: Capa de acceso a base de datos
- models: Capa de entidades y DTO
- db: Capa de conexion a Mongo db
- controller: Recibe las órdenes del router y se encarga de solicitar los datos al service
- api_caller: Capa que consume las Apis Externas

Obtenemos los siguientes datos de la pagina: https://www.mininter.gob.pe/ubica-tu-comisaria

- Listado de departamentos
- Listado de provincias
- Listado de distritos
- Listado de comisarias

Todos los datos son registrados en mongodb.

## PreRequisitos:
- Instalar RUST https://www.rust-lang.org/es/tools/install
- Crear base datos mongo, se recomienda utilizar la capa gratuita: https://www.mongodb.com/atlas/database
  - Crear la base de datos: **comisaria**
  - Crear las colecciones:
    - comisaria: registro de comisarias
    - departamento: registro de departamentos
    - provincia: registro de provincias
    - distrito: registro de distritos

## Configuracion:
- Editar el archivo .env:
  - Asignar en la variable DATABASE_CONNECTION tu conexion a tu base de datos.
  - La variable SERVER_PORT contiene el numero del puerto (por defecto es 8500)

## Ejecutar aplicacion.
```
cargo build
cargo run
```

## ENPOINTS:
1. Listado y registro de departamentos
```
curl -v http://localhost:8500/api/v1/ubigeo/getalldpto
```
2. Listado y registro de provincias
```
curl -v http://localhost:8500/api/v1/ubigeo/getallprov
```
3. Listado y registro de distritos(tener paciencia, es un proceso que consume mucho tiempo)
```
curl -v http://localhost:8500/api/v1/ubigeo/getalldist
```
4. Listado y registro de comisarias
```
curl -v http://localhost:8500/api/v1/comisaria/getall
```