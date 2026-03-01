use anchor_lang::prelude::*; 

declare_id!("");

#[program]
pub mod biblioteca{ 
    use super::*;

    pub fn crear_Biblioteca() -> Result<()>{

    }
}

#[account]
#[derive(InitSpace)]
pub struct Biblioteca{
    dueno: Pubkey, 

    #[max_len(60)]
    nombre: String,

    #[max_len(10)]
    libros: Vec<Libro>,
} 

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Libro{
    #[max_len(60)]
    nombre: String,

    paginas: u16, //Maximo de 65,535 paginas

    disponible: bool,
}

#[derive(Accounts)]
pub struct NuevaBiblioteca{
    #[account(mut)]
    pub dueno: Signer<'info>,

    #[account(
        init,
        player = dueno,
        space = Biblioteca::INIT_SPACE + 8;
        seddes = [b"biblioteca",dueno.key().as_ref()]
        bump
    )] 
    pub biblioteca: Account<'info, Biblioteca>,
    pub system_program:Program<'info, System>
}

pub struct NuevoLibro{
    pub dueno: Signer<'info>,

    #[account(mut)]
    pub biblioteca: Account<'info, Biblioteca>,
}
