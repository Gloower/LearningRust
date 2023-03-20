fn main() {

    let total_notas = 16;

    let media = calcular_media(total_notas);

    escreva_media(media);


}

fn calcular_media(total_notas: i32) -> i32 {
    total_notas / 4
}



fn escreva_media(media:i32) {
    if  media >= 7 {
        println!("Aprovado. Sua media foi de: {}", media)
    }

    else {
        println!("Reprovado. Sua media foi de: {}", media)
    }
}