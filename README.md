# 📖 WikiTranslate

Have you ever had troubles looking up the translation of a term with several ambiguities? I know I have. One of the best ways to overcome this (at least in my opinion) is to use the Wikipedia as your dictionary. You look up your term and use the linked articles in other languages as a base for translation.

That's what this tool does for you. Enter your term, choose the correct article and look at the translations. Easy as pie.

## Usage

`$ wt <lang-id> <search-term>`

e.g.

```
$ wt de lab
0) Lab
1) Kaspersky Lab
2) Lab-Farbraum
3) Lab (Begriffsklärung)
4) Skills Lab
5) Rocket Lab
6) MIT Media Lab
7) Linden Lab
8) FabLab
9) Ripple (Geldsystem)
 ...

Please enter the number of the result you want to translate.
0
Lab (als)
Cuallo (an)
منفحة (ar)
Quall (ca)
Pirrulu (co)
Syřidlo (cs)
Osteløbe (da)
Πυτιά (el)
Rennet (en)
Cuajo (es)
Laap (et)
 ...
```

## Instalation

`cargo install --git https://github.com/hoodie/wikitranslate-rs/`

## Shortcomings

This is hardly original, the original idea comes from [kiliankoe](https://github.com/kiliankoe/WikiTranslate).
