//! # Módulo de Conversão
//! 
//! Este módulo consiste em um Trait (similar a uma Interface) utilizado para adicionar um método
//!  que converte uma variável do tipo `f32` de radianos para graus, assim como sua implementação.
//! 
//! - Em Rust, não é possível adicionar métodos a tipos definidos por terceiros diretamente.
/// - O tipo `f32` é definido internamente pela linguagem, portanto, é necessário utilizar um Trait para
/// implementar novos métodos de forma indireta.

use mpu6050::PI;

/// Trait que possibilita a implementação do conversor de radianos para graus.
trait Conversion {
    fn to_degrees(&mut self) -> Self;
}

impl Conversion for f32 {
    /// Converte um valor em radianos para graus. <br>
    /// deg = rad * 180 / pi
    fn to_degrees(&mut self) -> Self {
        *self * 180.0 / PI
    }
}
