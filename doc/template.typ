#import "util.typ": hr-date-format

#let logo = () => {
  align(horizon, stack(dir: ltr,
    spacing: 5pt,
    image(
      "./FIDIT-logo.svg",
      height: 3cm
    ),
    align(left, text(font: "FreeSans", stack(
      spacing: 3pt,
      text(size: 5.5pt, spacing: 2pt)[Sveučilište u Rijeci],
      text(weight: "bold", size: 8pt, par(leading: 0.3em, [Fakultet informatike #linebreak() i digitalnih tehnologija])),
      v(5pt)
    ))),
  ))
}

#let title-page = () => [
#set text(size: 16pt)

#align(center)[

#logo()
#v(18pt)
Sveučilišni prijediplomski studij Informatika
#v(18pt * 3)

#text(size: 18pt)[Tin Švagelj]

#text(size: 28pt)[Metode rasterizacije volumetrijskih struktura u računalnoj grafici]

#text(size: 18pt)[Završni rad]
]

#v(18pt * 5)

*Mentor:* doc. dr. sc., Miran Pobar

#v(18pt * 8)

#align(center)[
Rijeka, #hr-date-format(datetime.today())
]

]

#let footer-format = (loc) => {
  if loc.page() < 5 {
    return
  }
  align(right, counter(page).display())
}

#let template = doc => {
  set page(
    paper: "a4",
    margin: 2.5cm,
  )

  title-page()

  set page(
    numbering: "1",
    footer: locate(footer-format)
  )
  
  set text(
    font: ("Times New Roman", "Liberation Serif"),
    size: 12pt,
  )

  set par(
    justify: true,
    leading: 0.15em + 1em * 0.25, // line height 1.15em - 0.15em + descent
  )
  show par: set block(
    spacing: 6pt + 1em * 0.25
  )
  
  set heading(numbering: "1.1.")
  show heading: set text(
    font: ("Arial", "Liberation Sans"),
  )
  show heading.where(level: 1): set text(size: 16pt)
  show heading.where(level: 1): it => {
    block(
      above: 0pt,
      below: 12pt,
      it
    )
  }
  show heading.where(level: 2): set text(size: 14pt)
  show heading.where(level: 2): it => block(
    above: 18pt,
    below: 6pt,
    it
  )
  show heading.where(level: 3): set text(size: 12pt)
  show heading.where(level: 3): it => block(
    above: 6pt,
    below: 6pt,
    it
  )
  show heading.where(level: 4): set text(size: 12pt)
  show heading.where(level: 4): it => block(
    above: 6pt,
    below: 6pt,
    it
  )
  
  show raw.line: set text(
    font: ("Consolas", "Courier New", "Liberation Mono"),
    size: 9pt,
  )

  set list(
    marker: "-",
  )

  // Uključuje ascent i descent u veličinu znaka za računanje razmaka
  // Ascent i descent ovise o fontu, ali obično su 50% fonta
  show heading: set block(inset: (y: 0.25em))
  show raw: set par(
    leading: 0.5em,
  )
  
  include "problem.typ"
  include "summary.typ"

  outline(
    title: "SADRŽAJ",
    indent: auto
  )

  pagebreak()
  counter(page).update(1)

  doc
}

// Naziv slike postaviti ispod same slike, a naziv tablice postaviti iznad tablice.
// Nazivi slika i tablica su centrirani, pismo Times New Roman, veličina 10 točaka.
