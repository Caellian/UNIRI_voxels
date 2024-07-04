#import "@preview/fletcher:0.4.2": *
#import "../util.typ": formula

= Oblici volumentrijskih podataka

Volumentrijski podaci se mogu predstaviti na nekoliko različitih načina gdje je područje primjene od velike važnosti za odabir idealne reprezentacije. Rasterizacija ovakvih podataka je generalno spora pa krivi odabir može učiniti izvedbu znatno kompliciranijom ili nemogučom.

Po definiciji iz teorije skupova, volumen tijela je definiran kao

#formula(
$
V := {(x,y,z) in RR | "uvijet za" (x,y,z)}
$
) <volumen>

gdje je uvijet jednadžba ili nejednadžba koja određuje ograničenja volumena, a $(x,y,z)$ uređena trojka koja predstavlja koordinate prostora u kojem se volumen nalazi.

S obzirom da se radi o skupu koji u trenutku prikaza mora biti određen, možemo ga aproksimirati i skupom unaprijed određenih točaka.

Iz toga slijedi da je bilo koja funkcija čija je kodomena skup _skupova uređenih trojki elemenata $RR$_ prikladna za predstavljanje volumetrijskih podataka.

Osim same pohrane informacije o postojanju neke točke $(x,y,z)$, u primjeni je bitna i pohrana podataka asociranih s tom točkom što se može matematički modelirati kao injektivna funkcija koja preslikava koordinate točke na sukladne podatke.

== Računalna pohrana volumena

Za pohranu volumentrijskih podataka u računalstvi su česte primjene:
- diskretnih podataka (unaprijed određenih vrijednosti) u obliku
  - nizova točaka ili "oblaka točaka" (engl. _point could_), ili
  - polja točaka (engl. _voxel grid_)
- jednadžbi pohranjenih u shaderima koje se koriste u konjunkciji s algoritmima koračanja po zrakama svijetlosti (engl. _ray marching_).

Diskretni podaci imaju jednostavniju implementaciju i manju algoritamsku složenost, no zauzimaju značajno više prostora u memoriji i na uređajima za trajnu pohranu. Za ray marching algoritme vrijedi obratno pa se ponajviše koriste za jednostavnije volumene i primjene gdje su neizbježni.

Definicija za volumen (@volumen) pruža korisno ograničenje jer pokazuje da možemo doći do volumetrijskih podataka i na druge načine. #linebreak() Primjer toga je česta primjena složenijih funkcija koje proceduralno generiraju nizove točaka za prikaz. Ovaj oblik uporabe je idealan za računalne igrice koje se ne koriste stvarnim podacima jer se oslanja na dobre karakteristike diskretnog oblika, a izbjegava nedostatak velikih prostornih zahtjeva na uređajima za trajnu pohranu.

#pagebreak()

= Strukture za pohranu volumetrijskih podataka

== 3D polja

```rust
const CHUNK_SIZE: usize = 32;
type EntryID = u16;

struct Chunk<T> {
  data: [[[EntryID; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
  values: Vec<T>
}
```

- Jednostavna i najčešća implementacija za real time render
- Postoji relativno puno primjera, alogritama, ...

== Stabla

#grid(
  columns: (3fr, 1fr),
  gutter: 1em,
)[
#lorem(50)
][
#diagram(
  node-stroke: .1em,
  node-fill: blue.lighten(80%),
  spacing: 1em,
  node((-0.2,0), `A`, radius: 1em),
  edge(),
  node((-1,1), `B`, radius: 1em),
  edge(),
  node((-1,2), `D`, radius: 1em),

  edge((-0.2,0), (0.7,1)),
  node((0.7,1), `C`, radius: 1em),
  edge(),
  node((0.1,2), `E`, radius: 1em),
  edge((0.7,1), (1.5,2)),
  node((1.5,2), `F`, radius: 1em),
)
]

=== Oktalna stabla

Oktalna stabla (engl. _octree_) su jedna od vrsta stabla koja imaju iznimno čestu uporabu u 3D grafici za ubrzavanje prikaza dijeljenjem prostora. Strukturirana podjela prostora dozvoljava značajna ubrzanja u ray tracing algoritmima jer zrake svijetlosti mogu preskočiti velike korake.

Koriste se i u simulaciji fizike jer dozvoljavaju brzo isključivanje tijela iz udaljenih dijelova prostora.

#grid(
  columns: (1fr, 1fr)
)[
```rust
enum Octree<T> {
  Leaf(T),
  Node {
    children: [Box<Octree>; 8],
  }
}
```
][
```rust
enum OctreeNode<T, const DEPTH: usize> {
  Leaf(T),
  Node {
    children: [Octree<T, {DEPTH - 1}>; 8],
  }
}
```
]


=== Raštrkana stabla voksela

Raštrkana stabla voksela (engl. _sparse voxel octree_, SVO) su vrsta stablastih struktura koja pohranjuje susjedne čvorove u nelinearnim segmentima memorije te zbog toga dozvoljava "prazne" čvorove.

```rust
enum Octree<T> {
  Leaf(Option<T>),
  Node {
    children: [Box<Octree>; 8],
  }
}
```

Prednost ovakvih struktura je što prazni dijelovi strukture ne zauzimaju prostor u memoriji, te ih nije potrebno kopirati u memoriju grafičkih kartica prilikom prikaza.

No iako rješavaju problem velike potrošnje memorije, čine izmjene podataka iznimno sporima te se zbog toga primjenjuju skoro isključivo za podatke koji se ne mijenjaju tokom provođenja programa.
Izvor loših performansi izmjena su potreba za premještanjem (kopiranjem) postojećih podataka kako bi se postigla njihova bolja lokalnost u međuspremniku (engl. _cache locality_) na grafičkoj kartici.

- Komplicirana implementacija
  - Postoji već gotov shader kod za ovo u par shading jezika negdje

- https://research.nvidia.com/sites/default/files/pubs/2010-02_Efficient-Sparse-Voxel/laine2010tr1_paper.pdf

=== DAG

- Varijanta SVOa, navesti razlike.

- Grozne karakteristike izmjena (po defaultu)
  - https://github.com/Phyronnaz/HashDAG

== Point-cloud data

Spremljeno u Octreeu zapravo?

Laserski skeneri ju generiraju, česta primjena u geoprostornoj analizi.
- Dronovi ju koriste za navigaciju.

#pagebreak()
