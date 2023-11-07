# Academic-groups

## Idea
A student buddy pairing where the idea is to pair people who want to learn to those who want to teach.

## Platforms
We use Rust programing language for back-end because it is considered a very good back-end programming language and a lot of people suggest to use this language over other alternatives.
For database we are using PostgreSQL because from the free and open-source options it is the best.
And all of this is written to be deployed in linux (we use fedora linux) where proffesionaly all the websites are hosted because of stability and open-source enviroment.

## Setup
1. Rust installation: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
2. For compiling and working with rust you will need to install these languages
  * Debian based: $ sudo apt install build-essential
  * Arch based: $ sudo pacman -S base-devel
  * Red hat linux based (Fedora): $ sudo dnf group install "C Development Tools and Libraries" "Development Tools"
3. Launch the PostgreSQL server in the Docker container in the terminal of the root directory by executing the command: docker-compose up -d
4. To install the SQLX CLI tool, run: cargo install sqlx-cli
5. To apply the “up” migration script to the PostgreSQL database use this command: sqlx migrate run
6. To install the necessary crates and launch the Actix Web HTTP server run: cargo r -r
7. Then website can be locally accesable in: http://127.0.0.1:8080/

P.S. Impossible to create full project in small time when there are other many projects in school
