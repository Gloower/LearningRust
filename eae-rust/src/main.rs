fn main(){

    let valor = 300.0;
    let icms = calcular_icms(valor);
    escreva_icms(icms);

}

fn calcular_icms(valor: f32) -> f32{
    valor * 17.0 / 100.0
}

fn escreva_icms(icms: f32) {
    println!("Icms: {}", icms);
}
