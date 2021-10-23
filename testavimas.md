---
breaks: false
---

# Testavimo planas

Pastabos:

* Žingsniai, kuriuos reikia įvykdyti yra nurodyti numeruotu sąrašu:
  1. Pirmas žingsnis.
  2. Antras žingsnis.
  3. …
* Dalykai, kuriuos reikia patikrinti yra nurodyti žymimaisiais laukeliais:
  * [ ] Pirmas.
  * [ ] Antras.
  * [ ] …

## Įdiegimas

1. Eikite į [Releases](https://github.com/vakaras/smauglys/releases/) puslapį ir
   pasirinkite naujausią versiją.
2. Jeigu norite testuoti 64 bitų versiją, atsisiųskite
   `smauglys_installer-x64-v-….exe`. Jei 32 bitų –
   `smauglys_installer-ia32-v-….exe`.
3. Atsisiuntus, tikėtina, kad naršyklė ir/ar antivirusinė programa nurodys, kad
   diegimo programa yra nepatikima ir pasiūlys ją ištrinti. Nurodykite, kad
   diegimo programa pasitikite.
4. Paleiskite diegimo programą.
5. Iššokusioje lentelėje nurodykite, kad norite leisti diegimo programai
   atlikti pakeitimus Jūsų kompiuteryje.
6. Sekite diegimo programos nurodymus ir įdiekite Smauglį.

* [ ] Diegimo programa turėtų baigti darbą be klaidų.
* [ ] Po sėkmingo įdiegimo, ant darbastalio turėtų atsirasti šaukinys `Smauglys`.

## Paleidimas

### Pirmasis paleidimas

Lietuvių kalbos paketai yra įdiegiami pirmojo paleidimo metu ir todėl lietuvių
kalbos vertimas yra suaktyvinamas tik antrojo paleidimo metu. Todėl, pirmą kartą
paleidę Smauglį, palaukite kol jis pilnai pasileis ir vėl uždarykite.

### Naudojantis darbastalio šaukiniu

* [ ] Ant darbastalio turėtų būti šaukinys `Smauglys`, kuriuo pasinaudojant
  galima paleisti Smauglį.

### Per kontekstinį meniu

* [ ] Failų naršyklėje dešiniuoju pelės klavišu paspaudus ant failo su `.py`
  plėtiniu, atsidariusiame kontekstiniame meniu turėtų būti pasirinkimas `Open
  with Smauglys`. Paspaudus ant jo, turėtų pasileisti Smauglys su redaktoriuje
  atvertu Python failu.

### Per PowerShell

* [ ] PowerShell įvedus komandą `smauglys` turėtų pasileisti Smauglys.

### Per komandų eilutę

* [ ] Windows „Command Prompt“ įvedus komandą `smauglys` turėtų pasileisti
  Smauglys.

## Naujo Python failo sukūrimas

1. Paleiskite Smauglį.
2. Sukurkite naują failą: *Failas* → *New File*.
3. Išsaugokite failą kaip `bandymas.py`: *Failas* → *Save* → pasirinkite aplanką
   kuriame norite išsaugoti → įveskite failo pavadinmą `bandymas.py` → *Save*.

* [ ] Apačioje esančios būsenos juostos kairiajame kampe turėtų atsirasti
  užrašas `Python 3.8.10`.

## Tipų tikrinimas

1. Paleiskite Smauglį ir sukurkite naują Python failą su tokia programa:

    ```python
    def klaida() -> None:
        eilutė: str = 4
        return eilutė
    ```

2. Išsaugokite failą (`CTRL+S` arba *Failas* → *Save*).

* [ ] `4` ir `return` turėtų būti pabraukti raudona vingiuota linija.
* [ ] Užvedus pele virš `4` turėtų parodyti pranešimą: `Incompatible types in assignment (expression has type “int”, variable has type “str”) mypy(error)`.
* [ ] Užvedus pele virš `return` turėtų parodyti pranešimą: `No return value expected mypy(error)`.
* [ ] Būsenos juostoje šalia kryžiuko turėtų būti skaičius `2`. Ant jo paspaudus, turėtų atverti `Problems` kortelę, kurioje turėtų būti rodomos dvi jau paminėtos klaidos.

## Programų vykdymas

Atverkite Smauglį ir sukurkite Python failą su tokia programa (2021 metų
informatikos VBE užduoties sprendimas):

```python
from typing import List, TextIO

class Pamoka:
    def __init__(self, pavadinimas: str):
        self.pavadinimas = pavadinimas
        self.mokiniai: List[str] = []
    def __repr__(self):
        return '{} {}'.format(self.pavadinimas, len(self.mokiniai))

def vidurkis(pažymiai: List[int]) -> float:
    suma = 0
    for pažymys in pažymiai:
        suma += pažymys
    return suma / len(pažymiai)

def rasti_arba_įdėti_pamoką(pamokos: List[Pamoka], pavadinimas: str) -> int:
    for (indeksas, pamoka) in enumerate(pamokos):
        if pamoka.pavadinimas == pavadinimas:
            return indeksas
    else:
        pamokos.append(Pamoka(pavadinimas))
        return len(pamokos) - 1

def skaityti(duomenų_failas: TextIO, pamokos: List[Pamoka]) -> None:
    for eilutė in duomenų_failas.readlines():
        (vardas, pavadinimas, pažymių_skaičius, *pažymiai_simb) = eilutė.split()
        pažymiai = list(map(int, pažymiai_simb))
        assert int(pažymių_skaičius) == len(pažymiai), \
            "Nuskaitytų pažymių kiekis nesutampa su nurodytu."
        if vidurkis(pažymiai) >= 9:
            indeksas = rasti_arba_įdėti_pamoką(pamokos, pavadinimas)
            pamokos[indeksas].mokiniai.append(vardas)

def mažiau(pamoka1: Pamoka, pamoka2: Pamoka) -> bool:
    return (
        len(pamoka1.mokiniai) > len(pamoka2.mokiniai) or
        (len(pamoka1.mokiniai) == len(pamoka2.mokiniai) and
            pamoka1.pavadinimas < pamoka2.pavadinimas)
    )

def rikiuoti(pamokos: List[Pamoka]) -> None:
    i = 0
    while i < len(pamokos)-1:
        mažiausias = i
        j = i + 1
        while j < len(pamokos):
            if mažiau(pamokos[j], pamokos[mažiausias]):
                mažiausias = j
            j += 1
        tarpinis = pamokos[mažiausias]
        pamokos[mažiausias] = pamokos[i]
        pamokos[i] = tarpinis
        i += 1

def spausdinti(rezultatų_failas: TextIO, pamokos: List[Pamoka]) -> None:
    if len(pamokos) == 0:
        print("Neatitinka vidurkis", file=rezultatų_failas)
    for pam in pamokos:
        print(pam.pavadinimas, len(pam.mokiniai), file=rezultatų_failas)
        for mok in pam.mokiniai:
            print(mok, file=rezultatų_failas)

def vykdyti(duomenų_failo_vardas: str, rezultatų_failo_vardas: str) -> None:
    pamokos: List[Pamoka] = []
    with open(duomenų_failo_vardas) as duomenų_failas:
        skaityti(duomenų_failas, pamokos)
    rikiuoti(pamokos)
    with open(rezultatų_failo_vardas, 'w') as rezultatų_failas:
        spausdinti(rezultatų_failas, pamokos)

vykdyti('U2_a.txt', 'U2rez_a.txt')
```

1. Paleiskite programą derinimo rėžimu: *Vykdyti* → *Start Debugging* (arba *F5*).
2. Iššokiusioje lentelėje *Debug Configuration* pasirinkite numatytąją opciją
   *Python file*.

   * [ ] Programa turėtų pasileisti ir eilutėje `with open(duomenų_failo_vardas) as
     duomenų_failas:` turėtų atsirasti pranešimas:

     ```plain
     Exception has occurred: FileNotFoundError
     [Errno 2] No such file or directory: 'U2_a.txt'
     ```

3. Sukurkite tekstinį failą `U2_a.txt` tame pačiame aplanke su tokiu turiniu:

   ```plain
   8
   Povilas Matematika 5 9 9 9 9 10
   Jonas Istorija 4 10 9 10 10
   Milda Matematika 6 10 8 9 9 9 9
   Asta Informatika 3 10 10 10
   Ieva Matematika 3 9 9 8
   Rima Informatika 3 10 10 7
   Juozas Fizika 2 8 9
   Laura Fizika 1 8
   ```

4. Vėl paleiskite programą.

   * [ ] Programa turėtų pasileisti ir eilutėje `(vardas, pavadinimas,
     pažymių_skaičius, *pažymiai_simb) = eilutė.split()` turėtų atsirasti
     pranešimas:

     ```plain
     Exception has occurred: ValueError
     not enough values to unpack (expected at least 3, got 1)
     ```

5. Paspauskite ant tos eilutės, kurioje įvyko klaida, kairiau jos numerio.

   * [ ] Turėtų atsirasti raudonas taškas rodantis, kad pavyko sukurti stabdos
     tašką.

6. Vėl paleiskite programą.

   * [ ] Programa turėtų pasileisti ir vykdymas turėtų sustoti eilutėje su
     stabdos tašku.

7. Kairėje ekrano pusėje esančioje `RUN AND DEBUG` skiltyje, kortelėje `WATCH`
   pridėkite `eilutė`.

   * [ ] Turėtų atsirasti toks užrašas: `eilutė: '8\n'`.

8. Paspauskite ant raudono tuščio kvadrato esančio lango viršuje tam, kad
   nutrauktumėte programos derinimą.

9. Paspauskite ant stabdos taško, kad jį pašalintumėte.

10. Pakeiskite funkciją `skaityti` į tokią:

    ```python
    def skaityti(duomenų_failas: TextIO, pamokos: List[Pamoka]) -> None:
        pirma_eilutė = True
        for eilutė in duomenų_failas.readlines():
            if pirma_eilutė:
                pirma_eilutė = False
                continue
            (vardas, pavadinimas, pažymių_skaičius, *pažymiai_simb) = eilutė.split()
            pažymiai = list(map(int, pažymiai_simb))
            assert int(pažymių_skaičius) == len(pažymiai), \
                "Nuskaitytų pažymių kiekis nesutampa su nurodytu."
            if vidurkis(pažymiai) >= 9:
                indeksas = rasti_arba_įdėti_pamoką(pamokos, pavadinimas)
                pamokos[indeksas].mokiniai.append(vardas)
    ```

11. Vėl paleiskite programą.

    * [ ] Programa turėtų sėkmingai užbaigti darbą.
    * [ ] Po įvykdymo, programos aplanke turėtų būti failas `U2rez_a.txt` su
      tokiu turiniu:

      ```plain
      Informatika 2
      Asta
      Rima
      Matematika 2
      Povilas
      Milda
      Istorija 1
      Jonas
      ```
