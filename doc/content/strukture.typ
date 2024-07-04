#import "@preview/fletcher:0.4.2": *

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
