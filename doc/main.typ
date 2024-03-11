#import "template.typ": template

#show: template

#include "./content/uvod.typ"
#include "./content/strukture.typ"
#include "./content/prijevremeno.typ"
#include "./content/realno_vrijeme.typ"
#include "./content/usporedba.typ"
#include "./content/prakticni.typ"
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
