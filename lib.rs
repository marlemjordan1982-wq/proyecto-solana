use anchor_lang::prelude::*;

declare_id!("DWX2Us5YV7EDKAp6p2vWxSgF8XjpPma4DRUux6zAZMFs");

#[program]
pub mod mi_contador {
    use super::*;

    pub fn crear_contador(ctx: Context<Initialize>) -> Result<()> {
        let contador = &mut ctx.accounts.contador;
        contador.valor = 0;
        Ok(())
    }

    pub fn incrementar(ctx: Context<Update>) -> Result<()> {
        let contador = &mut ctx.accounts.contador;
        contador.valor += 1;
        Ok(())
    }

    pub fn decrementar(ctx: Context<Update>) -> Result<()> {
        let contador = &mut ctx.accounts.contador;
        if contador.valor > 0 {
            contador.valor -= 1;
        }
        Ok(())
    }

    pub fn reiniciar(ctx: Context<Update>) -> Result<()> {
        let contador = &mut ctx.accounts.contador;
        contador.valor = 0;
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
pub struct Update<'info> {
    #[account(mut)]
    pub contador: Account<'info, Contador>,
}

#[account]
pub struct Contador {
    pub valor: u64,
}
