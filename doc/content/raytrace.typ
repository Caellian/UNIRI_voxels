= Alternativne metode prikaza

== Ray tracing

Svijetlost se može odbijati i ne završiti u kameri @Pharr2023-ex:
$
L_0(p, omega_0) = L_e (p, omega_0) + integral_(cal(S)^2) f(p, omega_0, omega_i) L_i (p,omega_i)|cos(theta_i)| d omega_i
$

Pojednostavljenje... promatramo zrake koje završe u kameri tako što simuliramo zrake _iz_ kamere i badabim, badapam zbrojimo konačan broj odskakanja, bla bla, itd.

Denoizing može biti savršen za voksele: Per-voxel Lighting via Path Tracing, Voxel Engine Devlog \#19 https://www.youtube.com/watch?v=VPetAcm1heI

== Ray marching

Koristimo neki jednostavan primitiv za testiranje kolizije zrake sa tijelom, obično kugla ili kocka. Koračamo za veličinu največeg takvog tijela unaprijed.

- Dozvoljava fora efekte poput metaballs.
- Brže od raytraceanja (u nekim slučajevima, npr. vokseli), no ograničenije jer zahtjeva da su sva prikazana tijela izražena formulama i shader onda mora rješavati sustave jednadžbi
  - Postoji negdje neki algoritam s kojim se može aproksimirati arbitrarni mesh ali daje dosta složene formule u mnogim slučajevima (koje su onda teške za predstaviti u kodu i spore za izračun).
    - Vidio sam ga prije 4/5 godina, treba to iskopati sada...
  - Je li doista brže za voksele (e.g. Nvidia SVO) ili samo za loše slučajeve? Slabo se koristi.

#pagebreak()