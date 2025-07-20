### 4.3 Derived Unit Atoms

**§29 dimensionless units** {#para-29} 

* 1. Dimensionless unit atoms are defined in [Table 3](#dimless).  
* 2. There are seven columns titled "name," "print," "c/s," "c/i," "M," "value," and "definition." The name is the full (official) name of the unit. The symbol recommended for use in print is listed in the column "print." "C/s," and "c/i" list the symbol in the case sensitive and the case insensitive variants respectively. The column "M" specifies whether this is a metric unit. The definition is a valid case sensitive expression of _The Unified Code for Units of Measure_ that defines the unit atom.  
* 3. Only the columns titled "c/s," "c/i," "M," "value," and "definition" are normative. Full name and print symbol are out of scope of _The Unified Code for Units of Measure_.  
* 4. The units named "parts per _N_" are provided to be used where absolutely necessary but are not endorsed. Especially "ppb" and "pptr" are deprecated since "billion" and "trillion" are ambiguous names internationally. The explicit powers of ten should be given instead.

#### Table 3: Dimensionless units. The units ppb and ppt are deprecated because the names "billion" and "trillion" are ambiguous. The expression "10*-9" or "10*-12" should be used instead. When the units percent or "parts per N" are used for concentrations specific units are preferred, e.g., "ug/l" for mass concentration. The expression "ug/kg" for ppb is also valid. {#dimless}

| name | print | c/s | c/i | M   | definition value | definition unit |
| --- | --- | --- | --- | --- | --- | --- |
| the number ten for arbitrary powers | 10n | `10*` | `10*` | no  | 10  | `1` |
| the number ten for arbitrary powers | 10n | `10^` | `10^` | no  | 10  | `1` |
| the number pi | π   | `[pi]` | `[PI]` | no  | π   | `1` |
| percent | %   | `%` | `%` | no  | 1   | `10*-2` |
| parts per thousand | ppth | `[ppth]` | `[PPTH]` | no  | 1   | `10*-3` |
| parts per million | ppm | `[ppm]` | `[PPM]` | no  | 1   | `10*-6` |
| parts per billion | ppb | `[ppb]` | `[PPB]` | no  | 1   | `10*-9` |
| parts per trillion | pptr | `[pptr]` | `[PPTR]` | no  | 1   | `10*-12` |

The notation "`10*`" for powers of ten originated in the HL7 "ISO+" extension of ISO 2955. In HL7 the character carat ('`^`') was thought as reserved. Since most people would expect to see "`10^3`" for the "third power of ten" and might in fact confuse "`10*3`" to mean "ten times 3", the symbol using the carat was later added to _The Unified Code for Units of Measure_.

**§30 SI units** {#para-30} 

* 1. SI units are defined by the international _Conférence Générale des Poids et Mesures_ (CGPM). _The Unified Code for Units of Measure_ definitions for those units are given in [Table 4](#si).  
* 2. There are seven columns titled "name," "print," "c/s," "c/i," "M," "value," and "definition." The name is the full (official) name of the unit. The symbol recommended for use in print is listed in the column "print." "C/s," and "c/i" list the symbol in the case sensitive and the case insensitive variants respectively. The column "M" specifies whether this is a metric unit. The definition is a valid case sensitive expression of _The Unified Code for Units of Measure_ that defines the unit atom.  
* 3. Only the columns titled "c/s," "c/i," "M," "value," and "definition" are normative. Full name and print symbol are defined by the CGPM and are out of scope of _The Unified Code for Units of Measure_.  
* 4. The function pair denoted "`cel(1 K)`" is defined as _f_C(_x_) = _x_ \- 273.15 to convert from kelvin to degree Celsius, and _f_C-1(_x_) = _x_ \+ 273.15 to convert from degree Celsius back to kelvin.

The case insensitive symbol for pascal is "`PAL`" which conforms to ISO 2955 and prevents the name conflict between pascal and pico-ampère.

Without reference to history, it is difficult to explain that the degree Celsius is part of the SI, because the degree Celsius is in a unique way incoherent with the SI, and is even superfluous since the base unit kelvin measures the same kind of quantity.

#### Table 4: SI units {#si}

| name | kind of quantity | print | c/s | c/i | M   | definition value | definition unit |
| --- | --- | --- | --- | --- | --- | --- | --- |
| mole | amount of substance | mol | `mol` | `MOL` | yes | 6.02214076 | `10*23` |
| steradian | solid angle | sr  | `sr` | `SR` | yes | 1   | `rad2` |
| hertz | frequency | Hz  | `Hz` | `HZ` | yes | 1   | `s-1` |
| newton | force | N   | `N` | `N` | yes | 1   | `kg.m/s2` |
| pascal | pressure | Pa  | `Pa` | `PAL` | yes | 1   | `N/m2` |
| joule | energy | J   | `J` | `J` | yes | 1   | `N.m` |
| watt | power | W   | `W` | `W` | yes | 1   | `J/s` |
| ampère | electric current | A   | `A` | `A` | yes | 1   | `C/s` |
| volt | electric potential | V   | `V` | `V` | yes | 1   | `J/C` |
| farad | electric capacitance | F   | `F` | `F` | yes | 1   | `C/V` |
| ohm | electric resistance | Ω   | `Ohm` | `OHM` | yes | 1   | `V/A` |
| siemens | electric conductance | S   | `S` | `SIE` | yes | 1   | `Ohm-1` |
| weber | magnetic flux | Wb  | `Wb` | `WB` | yes | 1   | `V.s` |
| degree Celsius | temperature | °C  | `Cel` | `CEL` | yes | •   | `cel(1 K)` |
| tesla | magnetic flux density | T   | `T` | `T` | yes | 1   | `Wb/m2` |
| henry | inductance | H   | `H` | `H` | yes | 1   | `Wb/A` |
| lumen | luminous flux | lm  | `lm` | `LM` | yes | 1   | `cd.sr` |
| lux | illuminance | lx  | `lx` | `LX` | yes | 1   | `lm/m2` |
| becquerel | radioactivity | Bq  | `Bq` | `BQ` | yes | 1   | `s-1` |
| gray | energy dose | Gy  | `Gy` | `GY` | yes | 1   | `J/kg` |
| sievert | dose equivalent | Sv  | `Sv` | `SV` | yes | 1   | `J/kg` |

**§31 other units from ISO 1000, ISO 2955 and ANSI X3.50** {#para-31} 

* 1. Those unit atoms listed by ISO 2955 under the heading "other units from ISO 1000" and some units from ANSI X3.50 are defined in [Table 5](#iso1000).  
* 2. The meaning of the columns is declared in [§30](#para-30).2\. 
* 3. Only the columns titled "c/s," "c/i," "M," "value," and "definition" are normative. Full name and print symbol are defined by ISO 1000 and are out of scope of _The Unified Code for Units of Measure_.

#### Table 5: Other units from ISO 1000, ISO 2955, and some from ANSI X3.50. {#iso1000}

| name | kind of quantity | print | c/s | c/i | M   | definition value | definition unit |
| --- | --- | --- | --- | --- | --- | --- | --- |
| gon, grade | plane angle | g   | `gon` | `GON` | no  | 0.9 | `deg` |
| degree | plane angle | °   | `deg` | `DEG` | no  | 2   | `[pi].rad/360` |
| minute | plane angle | '   | `'` | `'` | no  | 1   | `deg/60` |
| second | plane angle | ''  | `''` | `''` | no  | 1   | `'/60` |
| liter | volume | l   | `l` | `L` | yes | 1   | `dm3` |
| liter | volume | L   | `L` | `L` | yes | 1   | `l` |
| are | area | a   | `ar` | `AR` | yes | 100 | `m2` |
| minute | time | min | `min` | `MIN` | no  | 60  | `s` |
| hour | time | h   | `h` | `HR` | no  | 60  | `min` |
| day | time | d   | `d` | `D` | no  | 24  | `h` |
| tropical year | time | at  | `a_t` | `ANN_T` | no  | 365.24219 | `d` |
| mean Julian year | time | aj  | `a_j` | `ANN_J` | no  | 365.25 | `d` |
| mean Gregorian year | time | ag  | `a_g` | `ANN_G` | no  | 365.2425 | `d` |
| year | time | a   | `a` | `ANN` | no  | 1   | `a_j` |
| week | time | wk  | `wk` | `WK` | no  | 7   | `d` |
| synodal month | time | mos | `mo_s` | `MO_S` | no  | 29.53059 | `d` |
| mean Julian month | time | moj | `mo_j` | `MO_J` | no  | 1   | `a_j/12` |
| mean Gregorian month | time | mog | `mo_g` | `MO_G` | no  | 1   | `a_g/12` |
| month | time | mo  | `mo` | `MO` | no  | 1   | `mo_j` |
| tonne | mass | t   | `t` | `TNE` | yes | 1 × 103 | `kg` |
| bar | pressure | bar | `bar` | `BAR` | yes | 1 × 105 | `Pa` |
| unified atomic mass unit | mass | u   | `u` | `AMU` | yes | 1.66053906660 × 10-24 | `g` |
| electronvolt | energy | eV  | `eV` | `EV` | yes | 1   | `[e].V` |
| astronomic unit | length | AU  | `AU` | `ASU` | no  | 149597.870691 | `Mm` |
| parsec | length | pc  | `pc` | `PRS` | yes | 3.085678 × 1016 | `m` |

In the case sensitive variant the liter is defined both with an upper case '`L`" and a lower case '`l`'. NIST \[63 FR 40338\] declares the upper case 'L' as the preferred symbol for the U.S., while in many other countries the lower case 'l' is used. In fact the lower case 'l' was in effect since 1879. A hundred years later in 1979 the 16th CGPM decided to adopt the upper case 'L' as a second symbol for the liter. In the case insensitive variant there is only one symbol defined since there is no difference between upper case 'L' and lower case 'l'.

The unit "are" competes with year for the symbol "a" not only in ISO 2955, and ANSI X3.50, but also in ISO 1000 as stating the official CGPM approved symbols. This is why the symbol for are is "`ar`" in _The Unified Code for Units of Measure_. ISO 2955 explicitly adds the unit atom "`ha`" for hectare, while "hectare" is just the correct spelling of the compositum of "hecto" and "are" and thus would not require a separate unit atom. Nevertheless, ISO 2955 in its case insensitive variant assigns "`ARE`" to the are and "`har`" to the hectare. This is obviously an anomaly which _The Unified Code for Units of Measure_ will not follow. As a metric unit, "`ar`" can be prefixed with "`h`" to yield "`har`"

ANSI X3.50 had two different series of symbols for the units of time, the ones from ISO 2955 as adopted by _The Unified Code for Units of Measure_ and the symbols "`yr`" "`mo`" "`wk`" "`hr`" and "`sec`" while "`d`" and "`min`" were defined twice. _The Unified Code for Units of Measure_ does not define these synonyms of ISO 2955 symbols, but does adopt those units from ANSI X3.50 that are not part of ISO 2955, namely "`mo`" and "`wk`" Month and week are useful units mainly in business or clinical medicine.

The semantics of the units of time is difficult to capture. The difficulties start with the day: There is the sidereal and the solar day that depend on the earth's rotation. The earth's rotation is variable during one day and is continually slowing down in the long run. The usual subdivisions of the day in 24 hours of 60 minutes and 60 seconds originated in Babylonia. The earth's rotation was too inexact to measure time, which is why the 11th CGPM (1954) defined the second based on a standardized historical tropical year (see below) which was later (13th CGPM 1967-1968) replaced by frequency measurement. Thus the second came to be the base unit of time and the day is now 864000 s exactly with the _Universal Coordinated Time_ (UTC) adding leap seconds every now and then.

For the year we have to distinguish the "tropical" (solar, sidereal) year from the calendar year. And both are difficult. The tropical year is the year defined by time the earth travels around the sun. This is difficult to measure and varies over time. Around 1900 it was 365.242196 d, currently it is 365.242190 d and around 2100 it will be 365.242184 d. In addition these durations are averages. The actual length of each year may vary by several minutes due to the gravitational influence of other planets. Thus there is quite a high uncertainty already in the fourth decimal digit.

The calendar year is also difficult because there is the Julian calendar (Sosigenes of Alexandria and Julius Caesar, 45 BC) with a slightly too long year of 365.25 d that causes the calendar to be one day ahead of the tropical year in 128 years. The Gregorian calendar (Christopher Clavius 1537-1612 and Pope Gregory XIII 1545-1563) leaves out three leap years in 400 years (let _n_ be the year number, the leap year is dropped if _n_ mod 100 = 0 but not _n_ mod 400 = 0.) The Gregorian mean year is thus 365.2425 d. This leap year arithmetic seems to be too much even for astronomers, which is why the light year ends up being defined based on the Julian year \[NIST Sp. Pub. 811, 1995 Edition\]. For this reason _The Unified Code for Units of Measure_ defines Tropical, Julian and Gregorian year by means of subscripts, but assigns the default year symbol to the Julian year.

The week is 7 days, this is a biblic truth we can count on (it is actually quite plausible that the week of seven days originated in Babylonia and entered Jewish tradition during the Babylonian exile.)

The difficulty continues with the month. The lunar (so called "synodal" month is variable. Around 1900 it was 29.5305886 d currently it is 29.5305889 d and in 2100 it will be 29.5305891 d, which we fixate in the 5th decimal digit with a considerable uncertainty. The calendar month is difficult because of the uneven distribution of days in a month over the year, and because of the two different calendar years. But we will usually use the mean calendar month, which is the Julian calendar year divided by 12.

As a conclusion, great care has to be taken when the "customary units" of time are used to measure time. The SI has fixated the second which should be used whenever accuracy is required. For business purposes the Julian calendar is sufficient especially since the notion of the Work-Day (vs. Holiday) is more important than the imprecision over 128 years. \[Sources: "Calendar" _Britannica Online._`http://www.eb.com:180/cgi-bin/g?DocF=macro/5000/98/toc.html`. Claus Tondering, _Frequently asked questions about calendars._ Part 1. 1998. `http://www.pip.dknet.dk/~c-t/calendar.faq1.txt`\]

**§32 natural units** {#para-32} 

* 1. Fundamental constants of nature and units derived from these constants are defined in [Table 6](#const).  
* 2. The meaning of the columns is declared in [§30](#para-30).2\.  
* 3. Only the columns titled "c/s," "c/i," "M," "value," and "definition" are normative. Full name and print symbol are defined by ISO 1000 and are out of scope of _The Unified Code for Units of Measure_.

#### Table 6: Natural units. {#const}

| name | kind of quantity | print | c/s | c/i | M   | definition value | definition unit |
| --- | --- | --- | --- | --- | --- | --- | --- |
| velocity of light | velocity | _c_ | `[c]` | `[C]` | yes | 299792458 | `m/s` |
| Planck constant | action | _h_ | `[h]` | `[H]` | yes | 6.62607015 × 10-34 | `J.s` |
| Boltzmann constant | (unclassified) | _k_ | `[k]` | `[K]` | yes | 1.380649 × 10-23 | `J/K` |
| permittivity of vacuum | electric permittivity | _ε0_ | `[eps_0]` | `[EPS_0]` | yes | 8.854187817 × 10-12 | `F/m` |
| permeability of vacuum | magnetic permeability | _μ0_ | `[mu_0]` | `[MU_0]` | yes | 1   | `4.[pi].10*-7.N/A2` |
| elementary charge | electric charge | _e_ | `[e]` | `[E]` | yes | 1.602176634 × 10-19 | `C` |
| electron mass | mass | _me_ | `[m_e]` | `[M_E]` | yes | 9.1093837139 × 10-31 | `kg` |
| proton mass | mass | _mp_ | `[m_p]` | `[M_P]` | yes | 1.67262192595 × 10-27 | `kg` |
| Newtonian constant of gravitation | (unclassified) | _G_ | `[G]` | `[GC]` | yes | 6.67430 × 10-11 | `m3.kg-1.s-2` |
| standard acceleration of free fall | acceleration | _gn_ | `[g]` | `[G]` | yes | 9.80665 | `m/s2` |
| standard atmosphere | pressure | atm | `atm` | `ATM` | no  | 101325 | `Pa` |
| light-year | length | l.y. | `[ly]` | `[LY]` | yes | 1   | `[c].a_j` |
| gram-force | force | gf  | `gf` | `GF` | yes | 1   | `g.[g]` |
| pound force | force | lbf | `[lbf_av]` | `[LBF_AV]` | no  | 1   | `[lb_av].[g]` |

This list is not complete. It does not list all constants but only those that are fundamental and from which many other constants can be derived. The source of this table is _The NIST Reference on Constants, Units, and Uncertainty_ Version 2.1, 21 May 1998. NIST Physics Laboratory. `http://physics.nist.gov/cuu/Constants/index.html`

In the base system of _The Unified Code for Units of Measure_, the general gas constant _R_ is identical to the Boltzmann constant _k_. In the SI both are related through _R_ = _k_ × _N_A, where _N_A = 6.02214076 × 1023 /mol is the Avogadro constant. Because _The Unified Code for Units of Measure_ defines the mole to be the dimensionless Avogadro number (number of particles in 1 g of 12C itself, there is no difference anymore if the Boltzmann constant is given as _k_ = 1.380649 × 1023 J/K or _R_ = 8.314511 J mol-1 K-1.

**§33 CGS units** {#para-33} 

* 1. The units of the older Centimeter-Gram-Second (CGS) system are defined in [Table 7](#cgs).  
* 2. The meaning of the columns is declared in [§30](#para-30).2\.  
* 3. Only the columns titled "c/s," "c/i," "M," "value," and "definition" are normative. Full name and print symbol are out of scope of _The Unified Code for Units of Measure_.

#### Table 7: CGS units {#cgs}

| name | kind of quantity | print | c/s | c/i | M   | definition value | definition unit |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Kayser | lineic number | K   | `Ky` | `KY` | yes | 1   | `cm-1` |
| Gal | acceleration | Gal | `Gal` | `GL` | yes | 1   | `cm/s2` |
| dyne | force | dyn | `dyn` | `DYN` | yes | 1   | `g.cm/s2` |
| erg | energy | erg | `erg` | `ERG` | yes | 1   | `dyn.cm` |
| Poise | dynamic viscosity | P   | `P` | `P` | yes | 1   | `dyn.s/cm2` |
| Biot | electric current | Bi  | `Bi` | `BI` | yes | 10  | `A` |
| Stokes | kinematic viscosity | St  | `St` | `ST` | yes | 1   | `cm2/s` |
| Maxwell | flux of magnetic induction | Mx  | `Mx` | `MX` | yes | 1 × 10-8 | `Wb` |
| Gauss | magnetic flux density | Gs, G | `G` | `GS` | yes | 1 × 10-4 | `T` |
| Oersted | magnetic field intensity | Oe  | `Oe` | `OE` | yes | 250 | `/[pi].A/m` |
| Gilbert | magnetic tension | Gb  | `Gb` | `GB` | yes | 1   | `Oe.cm` |
| stilb | lum. intensity density | sb  | `sb` | `SB` | yes | 1   | `cd/cm2` |
| Lambert | brightness | L   | `Lmb` | `LMB` | yes | 1   | `cd/cm2/[pi]` |
| phot | illuminance | ph  | `ph` | `PHT` | yes | 1 × 10-4 | `lx` |
| Curie | radioactivity | Ci  | `Ci` | `CI` | yes | 3.7 × 1010 | `Bq` |
| Roentgen | ion dose | R   | `R` | `ROE` | yes | 2.58 × 10-4 | `C/kg` |
| radiation absorbed dose | energy dose | RAD | `RAD` | `[RAD]` | yes | 100 | `erg/g` |
| radiation equivalent man | dose equivalent | REM | `REM` | `[REM]` | yes | 1   | `RAD` |

Although the CGPM "accepts" only very few CGS units "for use with the SI," CGS units are proper metric units. CGS units are still used in many physiological laboratories and in clinical diagnostics (e.g., cardiology). In addition CGS units acquired a special dignity as this was the system of units used by the great physicists of the early 20th century, Albert Einstein, Max Planck, and many others who worked on the scientific revolution that had quite a cultural impact.

The CGS system defined electric and magnetic phenomena differently which is why the units named "oersted" and "maxwell" have no proper SI counterpart. This table was compiled from various sources and is not complete and not very systematic. We therefore welcome suggestions and advice as to how this table could be completed.