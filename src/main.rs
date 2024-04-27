use std::io;

fn main()
{
    {
        let mut mensagem_usuario = String::new();

        println!("Digite algo: ");

        match io::stdin().read_line(&mut mensagem_usuario)
        {
            Ok(_) => println!("Sucesso, você digitou {}", mensagem_usuario.to_uppercase()),
            Err(e) => println!("Erro: {}", e)
        }
    }

    {
        let nome = "Fernando";
        match nome
        {
            "Fernando" => println!("nasceu em 1977"),
            "Fernanda" => println!("nasceu em 1998"),
            _ => println!("não sei quando nasceu")
        }
    }

    {
        let numero = 15;
        match numero
        {
            1 => println!("número é um"),
            2 => println!("número é dois"),
            3 | 4 => println!("número é três ou quatro"),
            5..=20 => println!("Número está entre 5 e 20"),
            _ => println!("não sei que numero é")
        }
    }
}
