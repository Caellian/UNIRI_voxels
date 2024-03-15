= Strukture za pohranu volumetrijskih podataka

- Staviti primjere struktura u obliku slika i `struct`

== 3D polja

- Jednostavna i najčešća implementacija za real time render
- Postoji relativno puno primjera, alogritama, ...

== Octree

Navesti raštrkanu i zbijenu/linearnu strukturu.

=== SVO

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
