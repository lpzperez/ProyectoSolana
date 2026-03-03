import * as anchor from "@coral-xyz/anchor";
import { web3 } from "@coral-xyz/anchor";

describe("Tienda Videojuegos", () => {

  it("Crear tienda y leer datos", async () => {

    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.TiendaVideojuegos;

    const [tiendaPda] = web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("tienda"),
        provider.wallet.publicKey.toBuffer(),
      ],
      program.programId
    );

    console.log("DIRECCIÓN DE LA TIENDA (PDA):", tiendaPda.toBase58());

    const txHash = await program.methods
      .crearTienda("Game4Life")
      .accounts({
        owner: provider.wallet.publicKey,
        tienda: tiendaPda,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Tx:", txHash);

    await provider.connection.confirmTransaction(txHash);

    console.log("Tienda creada correctamente");

    const tienda = await program.account.tienda.fetch(tiendaPda);

    console.log("Datos on-chain:");
    console.log("Owner:", tienda.owner.toString());
    console.log("Nombre:", tienda.nombre);

    const videojuegosLegibles = tienda.videojuegos.map((v: any) => ({
      ...v,
      precio: v.precio.toString(),
    }));

    console.log("Videojuegos:", videojuegosLegibles);

    if (tienda.nombre !== "Game4Life") {
      throw new Error("El nombre no coincide");
    }

  });

});
