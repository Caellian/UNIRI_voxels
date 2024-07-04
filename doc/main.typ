#import "template.typ": config

#show: config(
  [Završni rad],
  [Metode prikaza volumetrijskih struktura u računalnoj grafici],
  [Tin Švagelj],
  attributions: [
    *Mentor:* doc. dr. sc., Miran Pobar
  ],
  inserts: [
    #include "problem.typ"
    #include "summary.typ"
  ]
)

#include "./content/uvod.typ"
#include "./content/raytrace.typ"
#include "./content/strukture.typ"
#include "./content/prijevremeno.typ"
#include "./content/realno_vrijeme.typ"
#include "./content/animacije.typ"
#include "./content/usporedba.typ"
#include "./content/zakljucak.typ"

#bibliography(
  title: "Literatura",
  "references.bib",
  style: "ieee"
)
#pagebreak()
#heading(numbering: none)[Popis priloga]
#outline(
  title: none,
  target: figure
)
