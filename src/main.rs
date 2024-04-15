use csv; //Import csv crate
use std::error::Error; //Handle error
use std::fs::OpenOptions; //Lettura file
use promptly::prompt; //Input utente
// use std::io::Write;

fn main() {

    println!("Benvenuti nella registrazione Utente");

    if let Err(err) = user_registration() {
        eprintln!("Errore durante la registrazione: {}", err);
    }
}

fn user_registration()-> Result<(), Box<dyn Error>> {
    let mut login_session = Vec :: new();

    let name  : String = prompt("Inserisci il tuo nome")?;
    login_session.push(name);

    let surname  : String = prompt("Inserisci il tuo cognome")?;
    login_session.push(surname);

    let tax_code  : String = prompt("Inserisci il tuo codice fiscale")?;
    if tax_code.len() != 16 {
        return Err("Il codice fiscale deve essere di 16 caratteri!".into());
    }
    login_session.push(tax_code);

    let address  : String = prompt("Inserisci il tuo indirizzio fisico")?;
    login_session.push(address);

    let email  : String = prompt("Inserisci la tua mail")?;
    if ! email.contains('@'){
        return Err("In dirizzo email non valido".into());
    }
    login_session.push(email);

    let user_id  : String = prompt("Inserisci il tuo nome utente")?;
    login_session.push(user_id);

    let password_id  : String = prompt("Inserisci la tua password di almeno 7 caratteri")?;
    if password_id.len() < 7 {
        return Err("La password deve contenere almeno 7 caratteri".into());
    }
    login_session.push(password_id);

    let file = OpenOptions::new().write(true).create(true).open("UtentiRegistrati.csv")?;
    let mut writer = csv::Writer::from_writer(file);

    writer.write_record(&login_session)?;

    println!("Registrazione effettuata con successo");

    Ok(())
}
