# Medidor de inclinação utilizando MPU6050
---

## Cálculo da inclinação

## Como instalar
Segue as instruções para rodar o código:

### Setup do Hardware

#### Equipamento
- ESP32: o código configura a [camada de abstração de hardware](https://github.com/esp-rs/esp-hal) para ser usada especificamente com uma ESP32 padrão. É possível que necessite de alterações caso queira utilizar outro microcontrolador da família da ESP32.
- MPU6050: o código utiliza uma biblioteca específica para o MPU6050, não sendo possível utilizar um acelerômetro genérico.

#### Conexões
- SDA do MPU6050 no pino 21 da ESP32
- SCL do MPU6050 no pino 22 da ESP32

### Setup do Software/toolchain
Instale [Rust e Cargo](https://www.rust-lang.org/tools/install).

Siga as etapas na seção de instalação do livro [The Rust on ESP32 Book](https://esp-rs.github.io/book/).

- A aplicação foi feita para uma ESP32 (Xtensa), `no_std`: não é necessário instalar dependências específicas para RISC-V ou que utilizam a biblioteca `std`.

### Compilar e gravar o código
Após instalar as dependências, clone o repositório e tente 

Não esqueça de exportar as variáveis de ambiente. Por padrão,

```bash
# Exportar variáveis
## UNIX
. ~/export-esp.sh

## Windows
%USERPROFILE%\export-esp.ps1

# Baixar e compilar o código
git clone https://github.com/KaueMiziara/gs-fisica-inclinator3001

cd gs-fisica-inclinator3001

cargo build
```

- Note que o primeiro `build` pode demorar.

Se não houver erros, basta conectar a ESP32 ao computador e gravar o código. O seguinte comando utiliza o `espflash` para gravar o código em uma placa conectada à porta PORTA (mude para a utilizada; no meu caso, `/dev/ttyUSB0`) e abre o monitor serial.

```bash
cargo espflash flash -p PORTA --monitor
```
