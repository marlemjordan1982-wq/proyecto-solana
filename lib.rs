use anchor_lang::prelude::*;

declare_id!("DWX2Us5YV7EDKAp6p2vWxSgF8XjpPma4DRUux6zAZMFs");

#[program]
pub mod mi_contador {
    use super::*;

   
    pub fn crear_contador(ctx: Context<Initialize>) -> Result<()> {
        let contador = &mut ctx.accounts.contador;
        contador.valor = 0;
        msg!("Contador creado con éxito");
        Ok(())
    }

   
    pub fn leer_contador(ctx: Context<Read>) -> Result<u64> {
        let valor_actual = ctx.accounts.contador.valor;
        msg!("El valor actual del contador es: {}", valor_actual);
        Ok(valor_actual)
    }

    
    pub fn incrementar(ctx: Context<Update>) -> Result<()> {
        let contador = &mut ctx.accounts.contador;
        contador.valor += 1;
        Ok(())
    }

    
    pub fn eliminar_contador(_ctx: Context<Close>) -> Result<()> {
        msg!("Cuenta cerrada y renta devuelta al usuario");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub contador: Account<'info, Contador>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Read<'info> {
    pub contador: Account<'info, Contador>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub contador: Account<'info, Contador>,
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut, close = user)]
    pub contador: Account<'info, Contador>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct Contador {
    pub valor: u64,
}
