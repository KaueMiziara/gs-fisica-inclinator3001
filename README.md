# Medidor de inclinação utilizando MPU6050
---

## Cálculo da inclinação
O acelerômetro mede a intensidade da aceleração da gravidade em cada eixo (x, y e z), em 'g' (1g = 9,81 m/s²). <br>
- Os valores em cada eixo variam de 0g a 1g.

Para se calcular a inclinação do corpo ao qual o acelerômetro está conectado, deve-se utilizar os vetores referentes às leituras para obter o ângulo de inclinação.

### Obtendo os ângulos
Seja ```X```, ```Y``` e ```Z``` os vetores correspondentes à aceleração nos eixos x, y e z, respectivamente. <br>
- O MPU6050 está paralelo ao corpo acoplado ("para cima") quando ```X = Y = 0``` e ```Z = 1g```;
- O MPU6050 está na vertical ("em pé") quando ```X = Z = 0``` e ```Y = 1g```;
- O MPU6050 está na horizontal ("deitado") quando ```Y = Z = 0``` e ```X = 1g```.

Seja &alpha; o plano do acelerômetro (considerado perfeitamente plano por simplicidade). Seja &theta; o ângulo de inclinação (ângulo de &alpha; em relação ao solo em torno do eixo x). <br>

Seja ```s``` a soma entre os vetores ```X``` e ```Z```. Por trigonometria básica, temos que: <br>
- ```θ = arctan(Y / sqtr(X² + Z²))```

Analogamente, o ângulo &phi; de rotação (ângulo de &alpha; em relação ao solo em torno do eixo y) é dado por:
- ```φ = arctan(- X / sqrt(Y² + Z²))```

Utilizando o Framework do Arduino, estes ângulos teriam que ser calculados manualmente, porém, a biblioteca que manipula a MPU6050 em Rust possui um método para calculá-los. Da documentação:

```rust
    /// Roll and pitch estimation from raw accelerometer readings
    /// NOTE: no yaw! no magnetometer present on MPU6050
    /// https://www.nxp.com/docs/en/application-note/AN3461.pdf equation 28, 29
    pub fn get_acc_angles(&mut self) -> Result<Vector2<f32>, Mpu6050Error<E>> {
        let acc = self.get_acc()?;

        Ok(Vector2::<f32>::new(
            atan2f(acc.y, sqrtf(powf(acc.x, 2.) + powf(acc.z, 2.))),
            atan2f(-acc.x, sqrtf(powf(acc.y, 2.) + powf(acc.z, 2.)))
        ))
    }
```

### Demonstração
Estes ângulos descrevem a inclinação do acelerômetro em relação ao solo. No contexto deste projeto, permitem medir a inclinação do antebraço de uma pessoa utilizando uma smartshirt. <br>

O grupo postou um [vídeo](https://www.youtube.com/watch?v=fgXRD-v5Gzs) demonstrando o protótipo funcionando no YouTube.

## Como instalar
Segue as instruções para rodar o código:

### Setup do Hardware

#### Equipamento
- ESP32: o código configura a [camada de abstração de hardware](https://github.com/esp-rs/esp-hal) para ser usada especificamente com uma ESP32 padrão. É possível que necessite de alterações caso queira utilizar outro microcontrolador da família da ESP32.
- MPU6050: o código utiliza uma biblioteca específica para o MPU6050, não sendo possível utilizar um acelerômetro genérico.

#### Conexões
- **SDA** do MPU6050 no pino 21 da ESP32
- **SCL** do MPU6050 no pino 22 da ESP32

### Setup do Software/toolchain
Instale [Rust e Cargo](https://www.rust-lang.org/tools/install).

Siga as etapas na seção de instalação do livro [The Rust on ESP32 Book](https://esp-rs.github.io/book/), sendo a seção [RISC-V and Xtensa targets](https://esp-rs.github.io/book/installation/riscv-and-xtensa.html) a que contém as informações mais importantes.

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

Se não houver erros, basta conectar a ESP32 ao computador e gravar o código. O seguinte comando utiliza o `espflash` para gravar o código em uma placa conectada à entrada PORTA (mude para a porta utilizada; no meu caso, `/dev/ttyUSB0`) e abre o monitor serial.

```bash
cargo espflash flash -p PORTA --monitor
```
