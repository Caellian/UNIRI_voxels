#import "../util.typ": add-more

= Uvod

Cilj računalne grafike je deterministički prikaz trodimenzionalnog (3D) sadržaja na zaslonu računala. Kako bi se to postiglo, primjenjuju različiti algoritmi na strukturama podataka koje su zavisne o području primjene i ciljevima softvera. Tradicionalni način prikaza 3D sadržaja je predstavljanje takvog sadržaja korištenjem jednostavnih matematičkih tijela (engl. _primitives_) poput trokuta, linearna transformacija njihovog prostora, projekcija na plohu koja je uzorkovana (engl. _sampled_) pravilnom rešetkom (engl. _grid_) piksela, te konačno prikazana na ekranu.

Nakon inicijalnog razvoja grafičkih kartica (engl. _graphics processing unit_, GPU) sa fiksiranim dijelovima procesa prikaza (engl. _fixed function pipeline_, FFP), kasnije je razvijena i sljedeća generacija GPUa sa programabilnim procesom prikaza (engl. _programmable pipeline_). Takve grafičke kartice dozvoljavaju uporabu različitih programa (engl. _shader_) za prikaz i izračun. Shaderi su konstantnim napretkom postajali sve fleksibilniji te se danas primjenjuju u mnoge svrhe koje nisu usko vezane za grafiku, ali zahtjevaju visoku razinu konkurentnih izračuna, poput:
- simulacije,
- analize podataka,
- neuralne mreže,
- obrade multimedijskih podataka, te brojne druge.

Prikaz volumetrijskih struktura je jedna od takvih namjena za koje FFP često nije prikladan jer se oslanja na specijalizirane algoritme koji često ne rade s trokutima nego simuliraju zrake svijetlosti. #linebreak()
Cilj ovog rada je prikazati načela rada nekih od tih struktura i algoritama, kao i njihovih prednosti i područja primjene.

Razvoj grafike koja utilizira FFP je popratio razvoj i popularnost sukladnih formata za pohranu modela koji opisuju isključivo površinsku geometriju predmeta (engl. _mesh_). Za svrhe jednostavnog prikaza je taj oblik pohrane idealan, no nedostatan je za obradu trodimenzionalnih podataka te ju čini složenijom nego što je potrebno. Također je nedostatan i za primjene u simulacijama jer zahtjeva nadomještaj nedostatnih podataka o volumenu različitim aproksimacijama za koje traže sporu prethodnu obradu (engl. _preprocessing_). #linebreak()
Drugi cilj ovog rada je osvrnuti se na takve formate za pohranu trodimenzionalnih podataka i ukazati na uvedene neučinkovitosti zahtjevane njihovim nedostacima.

- https://www.sciencedirect.com/topics/computer-science/volumetric-dataset
- https://developer.nvidia.com/gpugems/gpugems/part-vi-beyond-triangles/chapter-39-volume-rendering-techniques
- https://web.cse.ohio-state.edu/~shen.94/788/Site/Reading_files/Leovy88.pdf

== Oblici volumentrijskih podataka

Volumentrijski podaci se mogu predstaviti na nekoliko različitih načina gdje je područje primjene od velike važnosti za odabir idealne reprezentacije. Rasterizacija ovakvih podataka je generalno spora pa krivi odabir može učiniti izvedbu znatno kompliciranijom ili nemogučom.

Po definiciji iz teorije skupova, volumen tijela je definiran kao

#figure(
$
V := {(x,y,z) in RR | "uvijet za" (x,y,z)}
$,
caption: "definicija za volumen"
) <volumen>

gdje je uvijet jednadžba ili nejednadžba koja određuje ograničenja volumena.

S obzirom da se radi o skupu koji u trenutku prikaza mora biti određen, možemo ga aproksimirati i skupom unaprijed određenih točaka.

Iz toga slijedi da je bilo koja funkcija čija je kodomena skup _skupova uređenih trojki elemenata $RR$_ prikladna za predstavljanje volumetrijskih podataka.

=== Računalna primjena

Konkretno u računalnoj znanosti su česte primjene:
- diskretnih podataka (unaprijed određenih vrijednosti) u obliku
  - nizova točaka ili "oblaka točaka" (engl. _point could_), ili
  - polja točaka (engl. _voxel grid_)
- jednadžbi pohranjenih u shaderima koje se koriste u konjunkciji s algoritmima koračanja po zrakama svijetlosti (engl. _ray marching_).

Diskretni podaci imaju jednostavniju implementaciju i manju algoritamsku složenost, no zauzimaju značajno više prostora u memoriji i na uređajima za trajnu pohranu. Za ray marching algoritme vrijedi obratno pa se ponajviše koriste za jednostavnije volumene i primjene gdje su neizbježni.

Definicija za @volumen pruža korisno ograničenje jer pokazuje da možemo doći do volumetrijskih podataka i na druge načine. #linebreak() Primjer toga je česta primjena složenijih funkcija koje proceduralno generiraju nizove točaka za prikaz. Ovaj oblik uporabe je idealan za računalne igrice koje se ne koriste stvarnim podacima jer se oslanja na dobre karakteristike diskretnog oblika, a izbjegava nedostatak velikih prostornih zahtjeva na uređajima za trajnu pohranu.

== Primjene volumetrijskih podataka

- Medicina - https://www.sciencedirect.com/topics/computer-science/volumetric-data
  - Rendgenska tomografija
  - Elektronski mikroskopi
    - (engl. _Transmission Electron Microscopy_, TEM) i (engl. _Scanning Transmission Electron Microscopy_, STEM)
  - DICOM format
- Geoprostorna analiza
- Prozivodnja (doi: 10.1117/12.561181)
  - Brzina prototipiranja
- Simulacije
- Računalne igre

== Komercijalni primjeri

- C4, by Terathon http://www.terathon.com/
  - discontinued
- https://voxelfarm.com/index.html
- https://gareth.pw/web/picavoxel/
- Euclideon
  - Koristi point cloud mislim 
  - claim: "Ne koristi GPU pipeline"

#pagebreak()
