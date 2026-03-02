use anchor_lang::prelude::*;

declare_id!("");

#[program]
pub mod biblioteca {
    use super::*; 

    pub fn crear_biblioteca(context: Context<NuevaBiblioteca>, nombre: String) -> Result<()> {

        let owner_id = context.accounts.owner.key();
        msg!("Owner id: {}", owner_id); 

        let libros: Vec<Libro> = Vec::new();

        context.accounts.biblioteca.set_inner(Biblioteca { 
            owner: owner_id,
            nombre,
            libros,
        });
        Ok(()) 
    }

    pub fn agregar_libro(context: Context<NuevoLibro>, nombre: String, paginas: u16) -> Result<()> {
        require!( 
            context.accounts.biblioteca.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner 
        ); 

        let libro = Libro { 
            nombre,
            paginas,
            disponible: true,
        };

        context.accounts.biblioteca.libros.push(libro); 

        Ok(()) 
    }

    pub fn eliminar_libro(context: Context<NuevoLibro>, nombre: String) -> Result<()> {
        require!( 
            context.accounts.biblioteca.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let libros = &mut context.accounts.biblioteca.libros; 

        for i in 0..libros.len() { 
            if libros[i].nombre == nombre { 
                libros.remove(i);
                msg!("Libro {} eliminado!", nombre); 
                return Ok(()); 
            }
        }
        Err(Errores::LibroNoExiste.into()) 
    }

    pub fn ver_libros(context: Context<NuevoLibro>) -> Result<()> {
        require!( 
            context.accounts.biblioteca.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        
        msg!("La lista de libros actualmente es: {:#?}", context.accounts.biblioteca.libros);
        Ok(()) 
    }

    
    pub fn alternar_estado(context: Context<NuevoLibro>, nombre: String) -> Result<()> {
        require!( 
            context.accounts.biblioteca.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let libros = &mut context.accounts.biblioteca.libros; 
        for i in 0..libros.len() { 
            let estado = libros[i].disponible;  

            if libros[i].nombre == nombre { 
                let nuevo_estado = !estado;
                libros[i].disponible = nuevo_estado;
                msg!("El libro: {} ahora tiene un valor de disponibilidad: {}", nombre, nuevo_estado);
                return Ok(()); 
            }
        }

        Err(Errores::LibroNoExiste.into()) 
    }

}


#[error_code]
pub enum Errores {
    #[msg("Error, no eres el propietario de la biblioteca que deseas modificar")]
    NoEresElOwner,
    #[msg("Error, el libro con el que deseas interactuar no existe")]
    LibroNoExiste,
}

#[account]  
#[derive(InitSpace)]  
pub struct Biblioteca { 
    owner: Pubkey,  

    #[max_len(60)] 
    nombre: String,

    #[max_len(10)] 
    libros: Vec<Libro>,
}

  
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Libro {
    #[max_len(60)]
    nombre: String,

    
    paginas: u16, 

    disponible: bool,
}

 
#[derive(Accounts)]  
pub struct NuevaBiblioteca<'info> {  
    #[account(mut)] 
    pub owner: Signer<'info>, 
    #[account(
        init,  
        payer = owner, 
        space = Biblioteca::INIT_SPACE + 8, 
        seeds = [b"biblioteca", owner.key().as_ref()], 
        bump   
    )]
    pub biblioteca: Account<'info, Biblioteca>,  

    pub system_program: Program<'info, System>,  
}

 
#[derive(Accounts)] 
pub struct NuevoLibro<'info> {
    pub owner: Signer<'info>,  

    #[account(mut)] 
    pub biblioteca: Account<'info, Biblioteca>, 
