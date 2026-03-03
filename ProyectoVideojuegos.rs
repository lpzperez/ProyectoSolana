use anchor_lang::prelude::*;

declare_id!("2NVVH3TCpvrBuqavHpPSqWncPgUr6kjTjGC2dgKTr7dz");

#[program]
pub mod tiendavideojuegos {
    use super::*;

    // Instrucción: Crear tienda
    pub fn crear_tienda(context: Context<Nuevatienda>, nombre: String) -> Result<()> {
        let owner_id = context.accounts.owner.key();
        msg!("Owner id: {}", owner_id);

        let videojuegos: Vec<Videojuego> = Vec::new();

        context.accounts.tienda.set_inner(Tiendavideojuegos {
            owner: owner_id,
            nombre,
            videojuegos,
        });
        Ok(())
    }

    // Instrucción: Agregar videojuego
    pub fn agregar_videojuego(
        context: Context<Nuevovideojuego>,
        nombre: String,
        precio: u16,
        version: u16,
        anio: u16,
        compania: String,
    ) -> Result<()> {
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let juego = Videojuego {
            nombre,
            precio,
            version,
            anio,
            compania,
            disponible: true,
        };

        context.accounts.tienda.videojuegos.push(juego);

        Ok(())
    }

    // Instrucción: Eliminar videojuego
    pub fn eliminar_videojuego(context: Context<Nuevovideojuego>, nombre: String) -> Result<()> {
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let juegos = &mut context.accounts.tienda.videojuegos;

        for i in 0..juegos.len() {
            if juegos[i].nombre == nombre {
                juegos.remove(i);
                msg!("Videojuego {} eliminado!", nombre);
                return Ok(());
            }
        }
        Err(Errores::VideojuegoNoExiste.into())
    }

    // Instrucción: Ver videojuegos
    pub fn ver_videojuego(context: Context<Nuevovideojuego>) -> Result<()> {
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        msg!(
            "La lista de videojuegos actualmente es: {:#?}",
            context.accounts.tienda.videojuegos
        );
        Ok(())
    }

    // Instrucción: Alternar estado
    pub fn alternar_estado(context: Context<Nuevovideojuego>, nombre: String) -> Result<()> {
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let juegos = &mut context.accounts.tienda.videojuegos;
        for i in 0..juegos.len() {
            if juegos[i].nombre == nombre {
                let nuevo_estado = !juegos[i].disponible;
                juegos[i].disponible = nuevo_estado;
                msg!(
                    "El videojuego: {} ahora tiene disponibilidad: {}",
                    nombre,
                    nuevo_estado
                );
                return Ok(());
            }
        }

        Err(Errores::VideojuegoNoExiste.into())
    }
}

#[error_code]
pub enum Errores {
    #[msg("Error, no eres el propietario de la tienda que deseas modificar")]
    NoEresElOwner,
    #[msg("Error, el videojuego con el que deseas interactuar no existe")]
    VideojuegoNoExiste,
}

#[account]
#[derive(InitSpace)]
pub struct Tiendavideojuegos {
    owner: Pubkey,

    #[max_len(60)]
    nombre: String,

    #[max_len(20)]
    videojuegos: Vec<Videojuego>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Videojuego {
    #[max_len(60)]
    nombre: String,
    precio: u16,
    version: u16,
    anio: u16,
    #[max_len(60)]
    compania: String,
    disponible: bool,
}

#[derive(Accounts)]
pub struct Nuevatienda<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = Tiendavideojuegos::INIT_SPACE + 8,
        seeds = [b"tienda", owner.key().as_ref()],
        bump
    )]
    pub tienda: Account<'info, Tiendavideojuegos>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Nuevovideojuego<'info> {
    pub owner: Signer<'info>,

    #[account(mut)]
    pub tienda: Account<'info, Tiendavideojuegos>,
}
