#import "util.typ": locale-date-format

#let logo = image(
  "./FIDIT-logo.png",
  height: 3cm
)

#let title-page = (
  study,
  kind,
  title,
  author,
  attributions,
  logo,
  location,
  locale: "hr"
) => [
#set text(size: 16pt)
#set align(center)

#grid(
  rows: (auto, 1fr, 1fr, auto),
  columns: 1fr,

  stack(
    spacing: 20pt,
    logo,
    study
  ),

  align(bottom, stack(
    spacing: 20pt,
    text(size: 18pt, author),
    text(size: 28pt, title),
    text(size: 18pt, kind)
  )),

  align(horizon+left, attributions),

  align(center)[
    #location, #locale-date-format(locale, datetime.today())
  ]
)]

#let footer-format = (loc) => {
  let doc-start = locate(<first-page>).position()
  if loc.page() < doc-start.page {
    return
  }
  align(right, text(
    weight: "regular",
    size: 12pt,
    counter(page).display()
  ))
}

#let config = (
  study: [Sveučilišni prijediplomski studij Informatika],
  kind,
  title,
  author,
  attributions: [],
  logo: logo,
  location: [Rijeka],
  locale: "hr",
  inserts: [],
) => {
  return doc => {
    set page(
      paper: "a4",
      margin: 2.5cm,
    )

    title-page(
      study,
      kind,
      title,
      author,
      attributions,
      logo,
      location,
      locale: locale,
    )

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
    // Ascent i descent ovise o fontu, ali obično su 50% ukupne visine glifa
    show heading: set block(inset: (y: 0.25em))
    show raw: set par(
      leading: 0.5em,
    )
    
    inserts

    outline(
      title: "SADRŽAJ",
      indent: auto
    )

    pagebreak()
    counter(page).update(1)

    [#metadata((tag: "location-marker")) <first-page>]

    doc
  }
}

// TODO:
// Naziv slike postaviti ispod same slike, a naziv tablice postaviti iznad tablice.
// Nazivi slika i tablica su centrirani, pismo Times New Roman, veličina 10 točaka.
