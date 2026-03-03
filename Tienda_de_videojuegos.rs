use anchor_lang::prelude::*;

declare_id!("FY22vxkKth9cghM7p1vb55A2u3rQhpBoW8Fmr1LZYoUC");

#[program]
pub mod tienda_videojuegos {
    use super::*;

    pub fn crear_tienda(context: Context<NuevaTienda>, nombre: String) -> Result<()> {

        if context.accounts.tienda.owner != Pubkey::default() {
            msg!("La tienda ya existe");
            return Ok(());
        }

        let owner_id = context.accounts.owner.key();

        context.accounts.tienda.set_inner(Tienda {
            owner: owner_id,
            nombre: nombre.clone(),
            videojuegos: Vec::new(),
        });

        msg!("Tienda creada correctamente");
        Ok(())
    }

    pub fn agregar_videojuego(
        context: Context<NuevoVideojuego>,
        titulo: String,
        version: String,
        precio: u64,
        categoria: String,
        stock: u16,
    ) -> Result<()> {

        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let juego = Videojuego {
            titulo,
            version,
            precio,
            categoria,
            stock,
            disponible: true,
        };

        context.accounts.tienda.videojuegos.push(juego);

        Ok(())
    }

    pub fn eliminar_videojuego(
        context: Context<NuevoVideojuego>,
        titulo: String
    ) -> Result<()> {

        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let videojuegos = &mut context.accounts.tienda.videojuegos;

        for i in 0..videojuegos.len() {
            if videojuegos[i].titulo == titulo {
                videojuegos.remove(i);
                msg!("Videojuego {} eliminado!", titulo);
                return Ok(());
            }
        }

        Err(Errores::VideojuegoNoExiste.into())
    }

    pub fn ver_videojuegos(context: Context<NuevoVideojuego>) -> Result<()> {

        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let tienda = &context.accounts.tienda;

        msg!("==================================");
        msg!("Tienda: {}", tienda.nombre);
        msg!("Total videojuegos: {}", tienda.videojuegos.len());
        msg!("==================================");

        for juego in &tienda.videojuegos {
            msg!("------------------------------");
            msg!("Titulo: {}", juego.titulo);
            msg!("Version: {}", juego.version);
            msg!("Precio: {}", juego.precio);
            msg!("Categoria: {}", juego.categoria);
            msg!("Stock: {}", juego.stock);
            msg!("Disponible: {}", juego.disponible);
        }

        Ok(())
    }

    pub fn modificar_videojuego(
        context: Context<NuevoVideojuego>,
        titulo: String,
        nuevo_precio: u64,
        nuevo_stock: u16,
    ) -> Result<()> {

        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let videojuegos = &mut context.accounts.tienda.videojuegos;

        for i in 0..videojuegos.len() {
            if videojuegos[i].titulo == titulo {
                videojuegos[i].precio = nuevo_precio;
                videojuegos[i].stock = nuevo_stock;
                msg!("Videojuego {} modificado!", titulo);
                return Ok(());
            }
        }

        Err(Errores::VideojuegoNoExiste.into())
    }

    pub fn alternar_disponibilidad(
        context: Context<NuevoVideojuego>,
        titulo: String
    ) -> Result<()> {

        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let videojuegos = &mut context.accounts.tienda.videojuegos;

        for i in 0..videojuegos.len() {
            if videojuegos[i].titulo == titulo {
                videojuegos[i].disponible = !videojuegos[i].disponible;
                msg!("Disponibilidad cambiada para {}", titulo);
                return Ok(());
            }
        }

        Err(Errores::VideojuegoNoExiste.into())
    }
}

#[error_code]
pub enum Errores {
    #[msg("No eres el propietario de la tienda")]
    NoEresElOwner,
    #[msg("El videojuego no existe")]
    VideojuegoNoExiste,
}

#[account]
#[derive(InitSpace)]
pub struct Tienda {
    pub owner: Pubkey,

    #[max_len(60)]
    pub nombre: String,

    #[max_len(20)]
    pub videojuegos: Vec<Videojuego>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Videojuego {

    #[max_len(40)]
    pub titulo: String,

    #[max_len(20)]
    pub version: String,

    pub precio: u64,

    #[max_len(40)]
    pub categoria: String,

    pub stock: u16,

    pub disponible: bool,
}

#[derive(Accounts)]
pub struct NuevaTienda<'info> {

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init_if_needed,
        payer = owner,
        space = Tienda::INIT_SPACE + 8,
        seeds = [b"tienda", owner.key().as_ref()],
        bump
    )]
    pub tienda: Account<'info, Tienda>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NuevoVideojuego<'info> {

    pub owner: Signer<'info>,

    #[account(mut)]
    pub tienda: Account<'info, Tienda>,
}
