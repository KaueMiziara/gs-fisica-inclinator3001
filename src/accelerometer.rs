//! # Módulo do Acelerômetro
//! 
//! Este módulo fornece funcionalidades para interagir com o acelerômetro do MPU6050.

use esp_println::println;
use hal::{i2c::I2C, peripherals::I2C0};
use mpu6050::Mpu6050;

/// Struct que representa os ângulos do acelerômetro, que descrevem sua inclinação e rotação.
pub struct AccelerometerAngles {
    /// Ângulo de inclinação do acelerômetro (eixo x), em radianos:
    /// - Valores positivos indicam uma inclinação para frente
    /// - Valores negativos indicam uma inclinação para trás
    pub pitch: f32,
    /// Ângulo de rotação do acelerômetro (eixo y), em radianos:
    /// - Valores positivos indicam uma rotação no sentido horário
    /// - Valores negativos indicam uma rotação no sentido anti-horário
    pub roll: f32,
}

impl AccelerometerAngles {
    /// Cria uma nova instância de `AccelerometerData` a partir dos dados lidos do MPU6050.
    /// - Em caso de erro na leitura do sensor, instancia o struct com os valores padrão `0.0`.
    pub fn new(mpu6050: &mut Mpu6050<I2C<'_, I2C0>>) -> Self {
        let (pitch, roll) = match mpu6050.get_acc_angles() {
            Ok(ang) => (ang[0], ang[1]),
            Err(_) => {
                println!("Erro ao ler os dados do acelerômetro");
                (0.0, 0.0)
            }
        };

        Self { pitch, roll }
    }

    /// Imprime na tela os dados lidos do acelerômetro.
    pub fn print_angles(&self) {
        println!("---");
        println!("Ângulo de Inclinação: {} rad = {}º", self.pitch, self.pitch.to_degrees());
        println!("Ângulo de Rotação: {} rad = {}º", self.roll, self.roll.to_degrees());
        println!("---");
    }
}
