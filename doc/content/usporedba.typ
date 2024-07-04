= Usporedba s poligonima

Zbog prethodnih ograničenja u računalnoj memoriji, uporaba voksela je značajno slabije istražena metoda pohrane modela i prikaza. Postojeći harver je optimiran za prikaz tradicionalnih

== Problemi

- Kako zaobliti svijet
  - Je li vrijedno spomena, vrlo specifično za računalne igre...
  - Postoji negdje efekt sa shaderima; ne rješava probleme topologije
  - https://www.youtube.com/watch?v=bJr4QlDxEME

=== Izgled

- Sampliranje uvijek narušava kvalitetu modela, ili zauzima previše memorije
  - Artisti su naviknuti raditi s trokutima
  - Nije bitno za proceduralan sadržaj
    - Marching cubes i djeca
- Vrlo teško modelirati nepravilne oblike

== Interaktivnost

- "Svijet nije maketa od papira nego ima volumen".
  - Postoje metode lažiranja nekih vrsta interakcija.
    - Dodavanje svake zahtjeva zaseban trud dok je voxel engine uniforman (veliki upfront cost, lakše dodavanje sadržaja)

== Košta

- Kako bi vokseli bili praktični u real time aplikacijama potrebno je jako puno rada u optimizaciji njihove strukture u GPU memoriji, rendering kodu, LOD sustavu, ...
- Nvidia paper je dobar početak ali ne zadovoljava mnogo zahtjeva modernih računalnih igara
  - Mislim da nema transparentnost?

#pagebreak()
