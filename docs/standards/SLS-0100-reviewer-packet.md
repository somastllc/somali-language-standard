# SLS-0100 Pilot Reviewer Packet

- **Prepared:** 2026-09-04
- **Status:** LQ1–LQ16 answered; QF1–QF2 require clarification; Batch 5 (LQ17–LQ20) awaits review. Eighteen IDs are assigned: seventeen reviewed records and one draft.
- **Standard:** [SLS-0100 Dictionary Standard](../../spec/lexicon/0100-dictionary-standard.md)
- **Evidence map:** [SLS-0100 Dictionary Evidence Map](SLS-0100-evidence-map.md)
- **Decision record:** [SLS-0100 Review Log](SLS-0100-review-log.md)

This packet turns representative dictionary cases into explicit
maintainer questions. The source text is evidence, while the proposed Somali
and English glosses are new Draft wording. Answers are recorded verbatim in the
review log. Only then may an approved entry receive an `sls:lex:` identifier and
move into `data/lexicon/core/`.

## How to answer

Review three or four candidates at a time. For each candidate:

1. approve the proposed values, or replace any value in your own words;
2. resolve every question marked **Decision needed**;
3. do not approve a plural, gender, definition, or loanword status merely
   because it looks predictable;
4. say “defer” when the evidence is insufficient.

## Batch 1 — straightforward noun forms (answered; MR-1–MR-4)

### LQ1 — `baabuur`

**Source evidence:** [`qaamuus/01-b.md`](../../resources/qaamuus/01-b.md)
records `baabuur m.l (-rro, m.dh)` and describes motorized land transport for
people and goods. The paired headword occurs in
[`madax-ereyo/01-b.md`](../../resources/madax-ereyo/01-b.md).

**Proposed record values:**

- word: `baabuur`
- part of speech: `magac`
- gender: `masculine`
- plural: `baabuurro`
- Somali gloss: `Gaadiid matoor ku socda oo rakaab ama xamuul qaada.`
- English definition: `A motor vehicle used to transport people or goods.`
- dialect: `so`
- loanword status: unresolved (`null`)

**Recorded decision:** `is_loanword: true`; origin reviewed as Arabic (*bābūr*),
ultimately Italian (*vapore*). Entry `sls:lex:000001` is now in the reviewed
pilot seed.

**Decision needed:** Are all proposed values correct? If you know the
loanword status and origin from reliable evidence, state them; otherwise leave
the status unresolved.

### LQ2 — `naag`

**Source evidence:** [`qaamuus/18-n.md`](../../resources/qaamuus/18-n.md)
records `naag m.dh (-go, m.l)` with three related descriptions, including an
adult woman and the female spouse in a married pair.

**Proposed record values:**

- word: `naag`
- part of speech: `magac`
- gender: `feminine`
- plural: `naago`
- sense 1 Somali gloss: `Qof dumar ah oo qaangaaray.`
- sense 1 English definition: `An adult woman.`
- sense 2 Somali gloss: `Haweeneyda ku jirta lamaane is qaba.`
- sense 2 English definition: `The female spouse in a married couple.`
- dialect: `so`
- loanword status: `false`

**Recorded decision:** approved as proposed. Entry `sls:lex:000002` is now in
the reviewed pilot seed.

**Decision needed:** Are both senses correct and distinct? Is `naago` the
lexical plural for this entry, and is the non-loanword decision safe?

### LQ3 — `nin`

**Source evidence:** [`qaamuus/18-n.md`](../../resources/qaamuus/18-n.md)
records `nin m.l (-iman, m.l)` with an adult-man reading, a commendatory
character reading, and a male-spouse reading.

**Proposed record values:**

- word: `nin`
- part of speech: `magac`
- gender: `masculine`
- plural: `niman`
- sense 1 Somali gloss: `Qof rag ah oo qaangaaray.`
- sense 1 English definition: `An adult man.`
- sense 2 Somali gloss: `Ninka ku jira lamaane is qaba.`
- sense 2 English definition: `The male spouse in a married couple.`
- dialect: `so`
- loanword status: `false`

**Recorded decision:** include the commendatory sense as a third sense. Entry
`sls:lex:000003` is now in the reviewed pilot seed.

**Decision needed:** Should the source's commendatory sense be included as a
third dictionary sense? Confirm the plural, singular gender, and loanword
status.

### LQ4 — `mindi`

**Source evidence:** [`qaamuus/17-m.md`](../../resources/qaamuus/17-m.md)
records `mindi m.dh (-iyo, m.l)`, describes a cutting tool with a blade and
handle, and gives `middi` after `ld`.

**Proposed record values:**

- word: `mindi`
- part of speech: `magac`
- gender: `feminine`
- plural: `mindiyo`
- Somali gloss: `Qalab gacan-qabsi iyo af wax gooya leh.`
- English definition: `A tool with a cutting edge and a handle.`
- dialect: `so`
- loanword status: unresolved (`null`)
- `middi`: retained for later variant review, not added as a synonym

**Recorded decision:** narrow the gloss with the cutting-use phrase and record
`middi` as a reviewed phonetic variant. Entry `sls:lex:000004` is now in the
reviewed pilot seed.

**Decision needed:** Is the proposed gloss broad enough to mean “knife” and
not every handled cutting tool? Should `middi` be a spelling variant, a
separate entry, or omitted from the first seed?

## Batch 2 — plural and sense boundaries (answered; MR-5–MR-12)

### LQ5 — `buug`

**Source evidence:** [`qaamuus/01-b.md`](../../resources/qaamuus/01-b.md)
records masculine singular `buug` and the compact sequence
`(buug, m.dh/-aag/-ag, m.l)`. SLS-0003 G11-R3 independently approves
`buug → buugag` as a reviewed plural contrast.

**Proposed record values:**

- word: `buug`
- part of speech: `magac`
- gender: `masculine`
- plural: `buugag`
- sense 1 Somali gloss: `Xaashiyo isku xiran oo dabool leh, qoraalna xambaarsan.`
- sense 1 English definition: `Bound pages with writing, forming a book.`
- sense 2 Somali gloss: `Xaashiyo madhan oo isku xiran, daboolna leh.`
- sense 2 English definition: `Bound blank pages forming a notebook.`
- dialect: `so`
- loanword status: unresolved (`null`)

**Decision needed:** Should `buugag` be the one core plural while `buug` and
`buugaag` are deferred as plural variants? Are “book” and “blank notebook” two
senses of this entry?

### LQ6 — `qalin`

**Source evidence:** [`qaamuus/14-q.md`](../../resources/qaamuus/14-q.md)
marks `qalin` masculine and gives three meanings: a writing instrument,
metallic money, and a livestock mark. It lists no plural in the entry.

**Proposed record values:**

- word: `qalin`
- part of speech: `magac`
- gender: `masculine`
- dialect: `so`
- sense 1 Somali gloss: `Qalab gacanta lagu qabsado oo qoraal lagu sameeyo.`
- sense 1 English definition: `A pen or similar pointed writing instrument.`
- loanword status: unresolved (`null`)

**Decision needed:** What is the reviewed plural? Should the other two source
meanings be included now? Do not use JSON `null` merely because the source
omits the plural; `null` is reserved for a reviewed absence of an ordinary
plural.

### LQ7 — `sonkor` and `sokor`

**Source evidence:** [`qaamuus/08-s.md`](../../resources/qaamuus/08-s.md)
defines `sokor` as the sweet processed substance and makes `sonkor` an `ld`
cross-reference to it. SLS-0003 G11-R4 and its completed maintainer review use
`sonkor` for the mass-noun contrast.

**Proposed record values:**

- preferred core word: `sonkor`
- part of speech: `magac`
- gender: `feminine`
- plural: reviewed absence for the ordinary mass reading (`null`)
- Somali gloss: `Walax macaan oo cunto ama cabbid lagu macaaneeyo.`
- English definition: `A sweet substance used to sweeten food or drink.`
- dialect: `so`
- loanword status: unresolved (`null`)
- `sokor`: retained for variant review

**Decision needed:** Should `sonkor` or `sokor` be the canonical core
headword? Is the other form a valid Standard Somali variant? Confirm that the
ordinary mass reading should use plural `null`.

### LQ8 — `biyo`

**Source evidence:** [`qaamuus/01-b.md`](../../resources/qaamuus/01-b.md)
has `biyo¹ m.l.w` with a general water sense and a reproductive-fluid sense,
then `biyo² m.l.w (kiim.)` with an encyclopedic chemistry description of H2O.

**Decision needed:** Should the general water and chemistry descriptions be
one lexeme/sense rather than separate homographs? Should the reproductive-fluid
meaning be another sense of the same word? Confirm the primary class, gender,
number behaviour, concise Somali glosses, and matching English definitions
before any record is proposed.

## Batch 3 — homographs and source cross-references (answered; MR-13–MR-20)

### LQ9 — `guri`

**Source evidence:** [`qaamuus/12-g.md`](../../resources/qaamuus/12-g.md)
records `guri¹` as a masculine noun meaning a home/house and `guri²` as a
transitive verb with different meanings.

**Proposed treatment:** create two entries with the same canonical word `guri`
and different permanent IDs: one `magac`, one `fal`. Do not include `¹` or `²`
in either `word` value.

**Decision needed:** Approve or correct this homograph split. For the noun,
confirm gender `masculine`, plural `guriyo`, and a concise Somali/English
definition. For the verb, state which meaning should lead the seed entry.

### LQ10 — `hooyo`

**Source evidence:** [`qaamuus/20-h.md`](../../resources/qaamuus/20-h.md)
has a cross-reference-only `hooyo¹ m.l.w eeg hoy` and the feminine entry
`hooyo² (-ooyin, m.l)` for “mother,” an affectionate address, and two
allocutive uses.

**Proposed treatment:** seed only the “mother” lexeme initially:

- word: `hooyo`
- part of speech: `magac`
- gender: `feminine`
- plural: `hooyooyin`
- Somali gloss: `Waalid dumar ah; hooyo.`
- English definition: `A female parent; a mother.`
- dialect: `so`
- loanword status: `false`

**Decision needed:** Is this definition correct and sufficiently inclusive?
Should the affectionate and address uses become additional senses now? What
does the cross-reference-only `hooyo¹ → hoy` represent, and should it be
deferred?

### LQ11 — `kab`

**Source evidence:** [`qaamuus/15-k.md`](../../resources/qaamuus/15-k.md)
records masculine noun `kab¹` (a repair support/patch), feminine noun `kab²`
(footwear; plural `kabo`), and transitive verb `kab³` (to repair or join).

**Proposed treatment:** three distinct entries sharing canonical word `kab`,
each with its own ID and reviewed word class. Start the pilot with the footwear
noun only:

- gender: `feminine`
- plural: `kabo`
- Somali gloss: `Wax cagaha lagu xirto oo harag, caag, ama walxo kale laga sameeyo.`
- English definition: `An item of footwear made from leather, rubber, or similar material.`

**Decision needed:** Approve or correct the three-way split and the footwear
definition. Should `kab¹` and `kab³` enter this pilot or a later batch?

### LQ12 — `macallin`

**Source evidence:** [`qaamuus/17-m.md`](../../resources/qaamuus/17-m.md)
records masculine `macallin`, lists `-nno`, `-immo`, and `-imiin` plural
alternatives, and defines the referent specifically as a man who teaches
students.

**Proposed neutral sense wording:**

- Somali gloss: `Qof dadka aqoon ama xirfad bara.`
- English definition: `A person who teaches knowledge or a skill; a teacher.`

**Decision needed:** Does `macallin` itself remain grammatically masculine
when it refers to a woman, or should a feminine teacher use another headword?
Which plural should the core record use, and how should the other listed forms
be represented? Confirm whether the neutral definition changes the source
meaning correctly for present Standard Somali.

## Batch 4 — initial `b` nouns (answered; MR-21–MR-24)

These four original proposals are retained for auditability. MR-21–MR-24
record the maintainer's final values, including corrected plurals and gender;
the record values follow those answers rather than these earlier proposals.

### LQ13 — `baab`

**Source evidence:** [`qaamuus/01-b.md`](../../resources/qaamuus/01-b.md)
records `baab m.l (-bab, m.l)` with book-section and topic senses.

**Proposed record values:** masculine `magac`, plural `bab`, dialect `so`, and
loanword status unresolved (`null`). Proposed Somali gloss: `Qayb buug ka mid
ah ama mawduuc gaar ah.` Proposed English definition: `A section of a book or a
topic about a particular matter.`

**Decision needed:** Confirm gender, plural, senses, gloss, dialect, and
loanword status. Should `isku baab` be a separate sense or deferred?

### LQ14 — `baac`

**Source evidence:** [`qaamuus/01-b.md`](../../resources/qaamuus/01-b.md)
records `baac m.l (-cyo, m.dh)` for a measure between outstretched hands and a
related body-distance sense.

**Proposed record values:** masculine `magac`, plural `baacyo`, dialect `so`,
and loanword status unresolved (`null`). Proposed Somali gloss: `Cabbir u dhexeeya
labada gacmood marka la kala fidiyo.` Proposed English definition: `A measure of
length equal to the distance between outstretched hands.`

**Decision needed:** Are the two source meanings one sense or two? Confirm the
plural, gender, bilingual wording, and loanword status.

### LQ15 — `baabbul`

**Source evidence:** [`qaamuus/01-b.md`](../../resources/qaamuus/01-b.md)
records `baabbul m.dh (-lo, m.l)` with a lion's mane and long soft hair
readings.

**Proposed record values:** feminine `magac`, plural `baabbulo`, dialect `so`,
and loanword status unresolved (`null`). Proposed Somali glosses: `Dhogor cufan
oo ka baxda qaarka hore ee libaaxa` and `Timo jilicsan oo dheer.` Proposed
English definitions: `A lion's mane` and `Long, soft hair.`

**Decision needed:** Confirm the plural form and whether these are two senses
of one entry. Confirm the gender, definitions, dialect, and loanword status.

### LQ16 — `baaf`

**Source evidence:** [`qaamuus/01-b.md`](../../resources/qaamuus/01-b.md)
records `baaf m.l (-faf, m.l)` as a large metal basin used for washing clothes.

**Proposed record values:** masculine `magac`, plural `baafaf`, dialect `so`,
and loanword status unresolved (`null`). Proposed Somali gloss: `Weel weyn oo
bir ah oo dharka lagu dhaqo.` Proposed English definition: `A large metal basin
used for washing clothes.`

**Decision needed:** Confirm all proposed values and whether this is the only
core sense for the first seed.

## Consistency follow-up — October 2026

The questions below identify approval or alignment gaps in existing records.
The original maintainer answers remain unchanged in the review log.

### QF1 — exact `nin` commendatory gloss (D-7)

MR-3 supplied `nin rag ah` and the meaning “a brave, honorable, or capable
man.” The stored Somali gloss was expanded to `Nin rag ah oo geesi, sharaf
leh, ama karti leh.` That exact expansion has not been approved. The record
`sls:lex:000003` is now draft, retaining its ID and text.

**Decision needed:** Approve the exact stored Somali gloss, or supply its
replacement. The English definition is `A brave, honorable, or capable man.`
Is it aligned with your chosen Somali wording?

### QF2 — align the `guri` verb definitions (D-9)

MR-17 supplied Somali `Ururinta ama soo goynta miraha, dalagga, ama alaabta.`
and English `To gather, harvest, or pick fruits or crops.` Both remain exactly
as supplied in `sls:lex:000010`. The Somali includes `alaabta`; the English
mentions only fruits or crops.

**Decision needed:** Should the English definition include goods or objects,
or should the Somali definition have a narrower scope? Supply the exact pair
you want used, or explain why the existing pair should stand.

## Batch 5 — containers, sea, ice, and rice (unanswered)

These proposed values and new glosses require maintainer approval. No IDs are
allocated. All four candidates propose `part_of_speech: magac`, `dialect: so`,
and unresolved `is_loanword: null` unless the maintainer supplies a supported
classification. Confirm these shared fields along with each question.

### LQ17 — `baaldi`

**Source evidence:** [`qaamuus/01-b.md`](../../resources/qaamuus/01-b.md)
records `baaldi m.l (-iyo, m.dh)` and two container descriptions: one for
water and another for milk.

**Proposed values:** gender `masculine`; plural candidate `baaldiyo`.
Somali draft gloss: `Weel af ballaaran iyo siddo leh oo dareere lagu qaado.`
English draft definition: `A bucket with a wide opening and a handle, used
to carry liquids.`

**Decision needed:** Confirm or replace the plural, gender, gloss pair, and
loanword status. Should the water and milk uses be one sense or separate
senses? If separate, supply aligned definitions for both.

### LQ18 — `bad` (sea noun)

**Source evidence:** [`qaamuus/01-b.md`](../../resources/qaamuus/01-b.md)
records `bad¹ m.dh (-do, m.l)` with sea, card-playing-area, and poetic-meter
readings. Only the sea reading is proposed for this batch.

**Proposed values:** gender `feminine`; plural candidate `bado`.
Somali draft gloss: `Biyo milix leh oo ku fidsan qayb weyn oo dhulka ah.`
English draft definition: `A sea; a large body of salt water.`

**Decision needed:** Confirm or replace the plural, gender, definitions, and
loanword status. May the other readings be deferred while the first seed
records the sea sense alone?

### LQ19 — `baraf`

**Source evidence:** [`qaamuus/01-b.md`](../../resources/qaamuus/01-b.md)
records masculine `baraf`, plural notation `-fyo`, and frozen-water and very
cold readings. The source notation does not settle the scope of a mass versus
count reading.

**Proposed gloss for review:** Somali `Biyo qabow awgiis u adkaaday.` /
English `Ice; water frozen into a solid.` Gender candidate: `masculine`.

**Decision needed:** Does this reading take reviewed plural `null`, the lexical
plural `barafyo`, or a different value? Explain the reading to which that value
applies. Does this entry also cover snow, or only ice? Confirm definitions and
loanword status; do not derive a plural from the suffix without review.

### LQ20 — `bariis`

**Source evidence:** [`qaamuus/01-b.md`](../../resources/qaamuus/01-b.md)
records masculine `bariis`, describes the cultivated plant and its edible
grains, and lists `bariid` after `ld`. No plural is listed.

**Proposed gloss for review:** Somali `Badar la beerto oo iniinihiisa la
kariyo lana cuno.` / English `Rice; a cultivated grain cooked and eaten as
food.` Gender candidate: `masculine`.

**Decision needed:** Confirm gender, the ordinary plural value, the exact
gloss pair, and loanword status. Should plant, uncooked grain, and cooked rice
be separate senses? Should `bariid` be a reviewed variant or remain deferred?
An absent source plural does not itself authorize `null`.

## Approval effect

An approved answer does not modify the source files. It authorizes a newly
written, provenance-bearing lexicon record. Deferred or corrected values remain
visible in the review log, and no candidate receives an ID until all fields
required for that record are settled.
