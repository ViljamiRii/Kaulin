<img src="https://github.com/ViljamiRii/Kaulin/assets/125034560/f485cd23-ce19-4442-893e-d5fcf252f5f0"/>

> Kaulin on vielä kehitysvaiheessa! Julkaisu on Alpha versiossa ja ohjelmasta tulee löytymään bugeja.

> Kaulin is currently in development! The release is in Alpha and there will be bugs.

# Johdanto
Kaulin on Rust:lla tehty ohjelmointikieli jonka tein, koska tahdoin oppia lisää rustista. Kaulin skriptit toimivat tiedostopäätteellä .ka. Kielen syntaxi on sen uniikein elementti. 
Kieli ja sen syntaxi on tarkoitettu kirjoitettavaksi kokonaan Suomen kielellä. 
> # Introduction
> Kaulin is Rust made programming language which I made to familiarize my self with the language. Kaulin scripts have the file extension .ka. The syntax of the language is it's most unique element as
> the language and it's syntax is meant to be written completely in Finnish. If you want to learn finnish while using a half built language, this is a great option!

### Ennen Kaulimen käyttöä:
Kieli ei ole millään tavalla valmis, eikä luultavasti ikinä tule olemaan. Olen kielen ainoa kehittäjä ja teen kieltä vapaa-ajallani koulun vieressä. Kielen tarkoitus oli olla opettavainen kokemus minulle ja se täytti tarkoituksensa.
Jonkin näköinen dokumentaatio on tulossa kunhan kerkeän.
> ### Before using
> The language is in no way complete and will probably never be. I am the only developer working on this language and I'm doing it in my free time next to school. The goal of this project was to be an educational experience for me and it has served it's purpose.
> A documentation of some kind is coming along when I have time.

# Asennus
Itselläni on käytössä MacOS, joten asennus muille käyttöjärjestelmille ei vielä ole tiedossa. Päivitän tiedot samantien asennukseen kunhan saan käsiini windows ja linux laitteen.
> # Installation
> I'm using MacOS myself so the installation on other operating systems is still not clear to me. I will update the information as soon as I get my hands on a windows and a linux device.

### MacOS
1. Lataa [uusin versio](https://github.com/ViljamiRii/Kaulin/releases/tag/Kaulin) ohjelmasta.
2. Voit suorittaa .ka tiedoston avaamalla terminaalin ja käyttämällä komennon:
```
$ ~/Build/Kaulin main.ka
```
>HUOM! Tässä oletus on se, että Build tiedosto on polulla /Users/***käyttäjä***/Build
3. Voit suorittaa myös Kaulin tiedoston, jos haluat käyttää vain Repl muotoa.

>### MacOs
>1. Download the [newest version](https://github.com/ViljamiRii/Kaulin/releases/tag/Kaulin) of the program.
>2. You can run .ka file by opening your terminal and using the command:
>```
>$ ~/Build/Kaulin main.ka
>```
>NOTE! This is in the assumption that you have the build file under /Users/***current_user***/Build                                                              
>3. You can also run the Kaulin executable if you only want to use Repl.

### Esimerkki koodi:
>Example code:
```rust
//Kommentit toimivat kirjoittamalla // riville.
/*
Kieli myös tukee monen rivin
kommentteja kirjoittamalla kommentin näin.
*/

funktio laske_miljoonaan() {
    olkoon aloitus_aika = aika();
    toista ( olkoon i = 0; i < 100000000; i += 1 ) {
        // passes automatically
    }
    olkoon lopetus_aika = aika();
    olkoon koko_kesto = lopetus_aika - aloitus_aika;
    tulosta("Kesto: %{} ms", koko_kesto)
}

laske_miljoonaan()

Kesto: 5394 ms
```
### Sama pythonilla:
>Same with python:
```py
import time

def laske_miljoonaan():
    aloitus_aika = time.time() * 1000 
    for i in range(100_000_000):
        pass 
    lopetus_aika = time.time() * 1000 
    koko_kesto = lopetus_aika - aloitus_aika
    koko_kesto = round(koko_kesto)
    print(f"Kesto: {koko_kesto} ms")

laske_miljoonaan()

Kesto: 1345 ms
```
Tästä voi huomata, että kieli on huonosti optimoitu, eikä sitä kannata käyttää raskasta laskemista varten. Python on noin 4 kertaa nopeampi.
> As we can see from this example, the language is poorly optimized and should not be used for heavy calculations. Python is about 4 times faster.

# Syntax korostus
>Syntax highlighting

Kielelle on luotu syntax korostus Visual Studio Codea varten. Voit lisätä syntax korostuksen lataamalla [Github Repository](https://github.com/ViljamiRii/Kaulin/archive/refs/heads/master.zip)n.
Lataamisen jälkeen mene Kaulin-master tiedostoon terminaalilla ja käytä komento:
>I have implemented simple syntax highlighting for Visual Studio Code. You can add syntax highlighting by downloading the [Github Repository](https://github.com/ViljamiRii/Kaulin/archive/refs/heads/master.zip).
>After you have downloaded the repository, change directory to Kaulin-master on your terminal and use the command: 
```
$ cp -r kaulin-syntax ~/.vscode/extensions
```

# Credits
- Thanks to [Tyler Laceby](https://github.com/tlaceby) for helping me with the [guide to interpreters](https://github.com/tlaceby/guide-to-interpreters-series).
