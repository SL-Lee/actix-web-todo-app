# actix-web-todo-app
![](https://user-images.githubusercontent.com/45960387/203633174-4678efa6-6392-40a3-a068-8292963fe9bf.jpeg)

![](https://user-images.githubusercontent.com/45960387/203633229-3397ce8b-e517-4220-a38a-de36cfb13c62.jpeg)

![](https://user-images.githubusercontent.com/45960387/203633263-dc6c7860-0923-4b34-90b2-a839c323c29a.jpeg)

A simple todo web application written using the following technologies/frameworks:

- **actix-web** as the backend framework
- **PostgreSQL** as the database
- **Svelte** as the JavaScript frontend framework
- **Halfmoon** as the CSS frontend framework

## Setup

### Frontend
Before running any of the commands below, make sure you are in the project root.

```powershell
# Install the dependencies for Svelte
npm install

# Build the JavaScript bundle
npm run build
```

### Database
Before running any of the commands below, make sure you are in the project root.

```powershell
# Set the `PQ_LIB_DIR` environment variable. See the notes below for more details.
$env:PQ_LIB_DIR = "C:\Program Files\PostgreSQL\14\lib"

# Install the `diesel_cli` binary (if you haven't done so already)
cargo install diesel_cli --no-default-features --features postgres

# Modify the `DATABASE_URL` environment variable accordingly -- once you are done, save the file and close the notepad window.
notepad .\.env

# Run the `diesel setup` command to create the database (if it doesn't exist)
diesel setup

# Run the `diesel migration run` command to create the schema
diesel migration run
```

> **Note regarding the `PQ_LIB_DIR` environment variable:**
>
> The `PQ_LIB_DIR` environment variable is required for compiling `pq-sys`, a dependency of `diesel_cli` and the `diesel` library used by the server. Therefore, this environment variable is required every time you compile the server, so you might want to set it in a more persistent location (e.g. in Windows, set it in the system environment variables).
>
> The code for setting the `PQ_LIB_DIR` environment variable assumes you have PostgreSQL 14.x installed at the default location on Windows -- if you don't, you'll need to change that line accordingly to point to the location of your PostgreSQL installation's libraries.
>
> As for why setting the `PQ_LIB_DIR` environment variable is required, here are the relevant links:
>
> - [postgresql - How to fix Rust diesel cli link libpq.lib error on install - Stack Overflow](https://stackoverflow.com/questions/62708607/how-to-fix-rust-diesel-cli-link-libpq-lib-error-on-install/65880244#65880244)
>
>     This answer mentions that setting the `PQ_LIB_DIR` environment solves the issue of the `diesel_cli` binary not being able to find the `libpq` library.
>
> - [pq-sys/README.md at master · sgrif/pq-sys · GitHub](https://github.com/sgrif/pq-sys/blob/master/README.md#building)
>
>     The README of the `pq-sys` repository itself mentions that the first method of finding the libpq library is to check whether the `PQ_LIB_DIR` environment variable is set, and if so, it will use its value. For some reason, this is the only method that will work on Windows -- so if the `PQ_LIB_DIR` environment variable is not set, trying to compile `pq-sys` will fail with a linking error.

### Server
Before running any of the commands below, make sure you are in the project root, and the `PQ_LIB_DIR` environment variable is set.

```powershell
# Compile the server
cargo build --release

# Download the required library file -- see the notes below for more details.
$baseName = "gettext0.21-iconv1.16-shared-64"
$fileName = "$baseName.zip"
$downloadUrl = "https://github.com/mlocati/gettext-iconv-windows/releases/download/v0.21-v1.16/$fileName"
curl $downloadUrl -o $fileName -L
Expand-Archive $fileName "temp"
Move-Item "temp\bin\libintl-8.dll" "target\release\libintl-9.dll"
Remove-Item "temp", $fileName -Recurse

# Run the server
cargo run --release
```

Once the server has started successfully, visit http://127.0.0.1:8080/ in your browser (you can customize the server URL by changing the value of the `SERVER_URL` environment variable in the `.env` file).

> **Note regarding the required library:**
>
> If you run the server without downloading the required library, running the server will intermittently fail with the exit code 3. The explanation, as well as the solution for this issue, for this can be found [here](https://github.com/diesel-rs/diesel/discussions/2947#discussioncomment-2025857).
