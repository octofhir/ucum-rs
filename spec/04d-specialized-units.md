### 4.5 Other Legacy Units

**§43 legacy units for heat and temperature** {#para-43} 

* 1. ** Older units of heat (energy) and temperature are defined in [Table 17](#heat). 
* 2. ** The meaning of the columns is declared in [§30](#para-30).
* 3. ** Only the columns titled "c/s," "c/i," "M," "value," and "definition" are normative. Full name and print symbol are either not standardized or standardized by other bodies and are out of scope of _The Unified Code for Units of Measure_. 
* 4. ** The function pair denoted "`degf(5 K/9)`" is defined as _f_F(_x_) = 9/5 _x_ - 459.67 to convert from kelvin to degree Fahrenheit, and _f_F-1(_x_) = 5/9 (_x_ + 459.67) to convert from degree Fahrenheit back to kelvin. 
* 5. ** The function pair denoted "`degre(5 K/4)`" is defined as _f_Ré(_x_) = 4/5 _x_ - 218.52 to convert from kelvin to degree Réaumur, and _f_Ré-1(_x_) = 5/4 (_x_ + 218.52) to convert from degree Réaumur back to kelvin.

#### Table 17: Other Units for Heat and Temperature {#heat}

| name | kind of quantity | print | c/s | c/i | M   | definition value | definition unit |
| --- | --- | --- | --- | --- | --- | --- | --- |
| degree Fahrenheit | temperature | °F  | `[degF]` | `[DEGF]` | no  | •   | `degf(5 K/9)` |
| degree Rankine | temperature | °R  | `[degR]` | `[degR]` | no  | 5   | `K/9` |
| degree Réaumur | temperature | °Ré | `[degRe]` | `[degRe]` | no  | •   | `degre(5 K/4)` |
| calorie at 15 °C | energy | cal15°C | `cal_[15]` | `CAL_[15]` | yes | 4.18580 | `J` |
| calorie at 20 °C | energy | cal20°C | `cal_[20]` | `CAL_[20]` | yes | 4.18190 | `J` |
| mean calorie | energy | calm | `cal_m` | `CAL_M` | yes | 4.19002 | `J` |
| international table calorie | energy | calIT | `cal_IT` | `CAL_IT` | yes | 4.1868 | `J` |
| thermochemical calorie | energy | calth | `cal_th` | `CAL_TH` | yes | 4.184 | `J` |
| calorie | energy | cal | `cal` | `CAL` | yes | 1   | `cal_th` |
| nutrition label Calories | energy | Cal | `[Cal]` | `[CAL]` | no  | 1   | `kcal_th` |
| British thermal unit at 39 °F | energy | Btu39°F | `[Btu_39]` | `[BTU_39]` | no  | 1.05967 | `kJ` |
| British thermal unit at 59 °F | energy | Btu59°F | `[Btu_59]` | `[BTU_59]` | no  | 1.05480 | `kJ` |
| British thermal unit at 60 °F | energy | Btu60°F | `[Btu_60]` | `[BTU_60]` | no  | 1.05468 | `kJ` |
| mean British thermal unit | energy | Btum | `[Btu_m]` | `[BTU_M]` | no  | 1.05587 | `kJ` |
| international table British thermal unit | energy | BtuIT | `[Btu_IT]` | `[BTU_IT]` | no  | 1.05505585262 | `kJ` |
| thermochemical British thermal unit | energy | Btuth | `[Btu_th]` | `[BTU_TH]` | no  | 1.054350 | `kJ` |
| British thermal unit | energy | btu | `[Btu]` | `[BTU]` | no  | 1   | `[Btu_th]` |
| horsepower | power |     | `[HP]` | `[HP]` | no  | 550 | `[ft_i].[lbf_av]/s` |
| tex | linear mass density (of textile thread) | tex | `tex` | `TEX` | yes | 1   | `g/km` |
| Denier | linear mass density (of textile thread) | den | `[den]` | `[DEN]` | no  | 1   | `g/9/km` |

The degree Fahrenheit was missing in ANSI X3.50. HL7's "ISO+/ANS+" code defined the degree Fahrenheit under the symbol "`DEGF`" which is reflected here. This is the reason why _The Unified Code for Units of Measure_ does not define a new symbol "`Fah`" similar to "`Cel`" of ISO 2955 for the degree Celsius.

Defining precise semantics for legacy units for "quantity of heat" is difficult. The many variants of these units are frequently confused because there is not just a calorie and not just a British thermal unit. The different calories usually being used vary by 1% but the confusion can result in an error as high as 100000%! Thus, if exactness and non-ambiguity is important one should use the joule to report amounts of heat, just like for any other energy and work kind-of-quantities.

The gram-calorie, sometimes called "small calorie" is defined as the amount of heat required to raise the temperature of 1 gram of Water from 14.5 °C to 15.5 °C. According to _Encyclopedia Britannica_, this is the calorie most often used in engineering. There is also a less frequently used gram-calorie at 19.5 °C to 20.5 °C and a mean calorie that is 1/100 of the amount of heat required to raise the temperature from 0 °C to 100 °C. The _International Table_ calorie is defined by the _International Conference on the Properties of Steam_ (1956) and is used in steam engineering. In chemistry a "thermochemical" calorie is used for reaction enthalpies.

To complete the confusion, there is also a kilogram-calorie ("large calorie" , that has a similar definition based on a kilogram instead of a gram of water. This kilocalorie has also been called "calorie" in the sloppy speech of everyday life about food. U.S. "Nutrition Facts" that label almost every American food say "Calories: xxx" The _International Union of Nutritional Sciences_ recommends using either the joule or a kilocalorie based on the thermochemical calorie. Because of a perceived popular demand _The Unified Code for Units of Measure_ defines the nutrition Calorie as "`Cal`" with the conventional capital first letter. For the case insensitive variant of _The Unified Code for Units of Measure_, the symbol is enclosed in square brackets ("`[CAL]`").

Only the International Table calorie and the thermochemical calorie have exact definitions. To give some guidance in the confusing plenty of different calories, _The Unified Code for Units of Measure_ defines a default symbol "`cal`" as an alias for the thermochemical calorie, because the calorie is mostly used today in medicine and biochemistry. On the other hand, we consider engineers smart enough to select the precise calorie they mean.

Similar to the calories, various "British Thermal Unit" (Btu) are defined and the confusion continues. One Btu is defined as the amount of heat necessary to raise the temperature of one avoirdupois pound of water by one degree Fahrenheit beginning from various temperatures (39 °F, 59 °F, or 60 °F). There is also the International Table Btu and the thermochemical Btu. Just as with the calorie we define a default symbol "`Btu`" as an alias for the thermochemical Btu.

**§44 units used predominantly in clinical medicine** {#para-44} 

* 1. Units used mainly in clinical medicine are defined in [Table 18](#clinical).  
* 2. The meaning of the columns is declared in [§34](#para-34).  
* 3. The function pair denoted "`hpX(1 l)`" is defined as _f_hp X(_x_) = - lg _x_ to convert from a number fraction (dilution) per liter to the homeopathic potency value of the decimal (X) series, and _f_hp X-1(_x_) = 10-_x_ to convert from the potency value back to the number fraction. Likewise, the function pair denoted "`hpC(1 l)`" is defined as _f_hp C(_x_) = - ln(_x_) / ln(100) to convert from a number fraction (dilution) per liter to the homeopathic potency value of the centesimal (C) series, and _f_hp C-1(_x_) = 100-_x_ to convert from the potency value back to the number fraction. Analogous functions are defined for the millesimal (M) series with basis 1,000 and the series and the quintamillesimal (Q) series with basis 50,000.  
* 4. The function pair denoted "`100tan(1 rad)`" is defined as _f_PD(_α_) = tan(_α_) × 100 to convert from a plane angle _α_ to a prism diopter value (or a slope percent value) and _f_PD-1(_x_) = arctan(_x / 100_) to convert from prism diopter (or slope percent) value _x_ back to a plane angle.

#### Table 18: Units Used Predominantly in Clinical Medicine {#clinical}

| name | kind of quantity | print | c/s | c/i | M   | definition value | definition unit |
| --- | --- | --- | --- | --- | --- | --- | --- |
| meter of water column | pressure | m H2O | `m[H2O]` | `M[H2O]` | yes | 9.80665 | `kPa` |
| meter of mercury column | pressure | m Hg | `m[Hg]` | `M[HG]` | yes | 133.3220 | `kPa` |
| inch of water column | pressure | in H2O | `[in_i'H2O]` | `[IN_I'H2O]` | no  | 1   | `m[H2O].[in_i]/m` |
| inch of mercury column | pressure | in Hg | `[in_i'Hg]` | `[IN_I'HG]` | no  | 1   | `m[Hg].[in_i]/m` |
| peripheral vascular resistance unit | fluid resistance | P.R.U. | `[PRU]` | `[PRU]` | no  | 1   | `mm[Hg].s/ml` |
| Wood unit | fluid resistance | Wood U. | `[wood'U]` | `[WOOD'U]` | no  | 1   | `mm[Hg].min/L` |
| diopter | refraction of a lens | dpt | `[diop]` | `[DIOP]` | no  | 1   | `/m` |
| prism diopter | refraction of a prism | PD  | `[p'diop]` | `[P'DIOP]` | no  | •   | `100tan(1 rad)` |
| percent of slope | slope | %   | `%[slope]` | `%[SLOPE]` | no  | •   | `100tan(1 rad)` |
| mesh | lineic number |     | `[mesh_i]` | `[MESH_I]` | no  | 1   | `/[in_i]` |
| Charrière, french | gauge of catheters | Ch  | `[Ch]` | `[CH]` | no  | 1   | `mm/3` |
| drop | volume | drp | `[drp]` | `[DRP]` | no  | 1   | `ml/20` |
| Hounsfield unit | x-ray attenuation | HF  | `[hnsf'U]` | `[HNSF'U]` | no  | 1   | `1` |
| metabolic equivalent | metabolic cost of physical activity | MET | `[MET]` | `[MET]` | no  | 3.5 | `mL/min/kg` |
| homeopathic potency of decimal series (retired) | homeopathic potency (retired) | X   | `[hp'_X]` | `[HP'_X]` | no  | •   | `hpX(1 1)` |
| homeopathic potency of centesimal series (retired) | homeopathic potency (retired) | C   | `[hp'_C]` | `[HP'_C]` | no  | •   | `hpC(1 1)` |
| homeopathic potency of millesimal series (retired) | homeopathic potency (retired) | M   | `[hp'_M]` | `[HP'_M]` | no  | •   | `hpM(1 1)` |
| homeopathic potency of quintamillesimal series (retired) | homeopathic potency (retired) | Q   | `[hp'_Q]` | `[HP'_Q]` | no  | •   | `hpQ(1 1)` |
| homeopathic potency of decimal hahnemannian series | homeopathic potency (Hahnemann) | X   | `[hp_X]` | `[HP_X]` | no  | •   | •   |
| homeopathic potency of centesimal hahnemannian series | homeopathic potency (Hahnemann) | C   | `[hp_C]` | `[HP_C]` | no  | •   | •   |
| homeopathic potency of millesimal hahnemannian series | homeopathic potency (Hahnemann) | M   | `[hp_M]` | `[HP_M]` | no  | •   | •   |
| homeopathic potency of quintamillesimal hahnemannian series | homeopathic potency (Hahnemann) | Q   | `[hp_Q]` | `[HP_Q]` | no  | •   | •   |
| homeopathic potency of decimal korsakovian series | homeopathic potency (Korsakov) | X   | `[kp_X]` | `[KP_X]` | no  | •   | •   |
| homeopathic potency of centesimal korsakovian series | homeopathic potency (Korsakov) | C   | `[kp_C]` | `[KP_C]` | no  | •   | •   |
| homeopathic potency of millesimal korsakovian series | homeopathic potency (Korsakov) | M   | `[kp_M]` | `[KP_M]` | no  | •   | •   |
| homeopathic potency of quintamillesimal korsakovian series | homeopathic potency (Korsakov) | Q   | `[kp_Q]` | `[KP_Q]` | no  | •   | •   |

Clinical medicine all over the world still uses  mm Hg to measure arterial blood pressure, and often the instruments used are real mercury columns. Likewise, the central venous blood pressure is often measured using simple water columns which is very practical for the routine. The units  m H2O and  m Hg are metric units even though they are "not accepted" for use with the SI for quite a while. Although more and more hospitals in Europe switch to using the pascal to measure partial pressures in blood gas analysis, the older units will not vanish any time soon.

In the U.S. the inch is sometimes used instead of the millimeter, and because the inch is non-metric the inch of mercury or water columns is non-metric as well.

The peripheral vascular resistance unit is the vascular resistance on which a perfusion pressure of 1 mm Hg causes a flow of 1 ml/s.

The "mesh" occurs in the NIST Guide to the SI. It seems like it is the customary counterpart of the diopter.

The unit "charrière" originates from a French manufacturer of medical instruments by that name. One charrière is the gauge of a catheter with a circumference of approximately 1 mm such that it is by convention exactly one third of a millimeter. In the U.S. the charrière is simply called "french"

> **NOTE:** Note that _Unified Code for Units of Measure_ versions prior to 1.9 defined this unit as 1/π, this use, however this was never common use of the unit. This is why the definition has been corrected instead of adding another one.

A drop is a variable amount of fluid and depends on the device and technique used to produce the drop and on the physical properties of the fluid. This is similar to units like cup, tablespoon, and teaspoon that depend on the spoon or cup and are not exact either. However, in clinical medicine medication is dispensed by drops and unlike a "tablet" a drop refers to a real physical kind of quantity, volume, though not very exact.

> **NOTE:** Note that _Unified Code for Units of Measure_ versions prior to 1.9 defined this unit as mL/12, this use, however this was not common use of the unit. This is why the definition has been corrected instead of adding another one. Typically it is stated as mL/20. Original research using a 20 mL syringe filled to 5 mL shows that 1 mL has 25 drops of water, when tensides are added, the number goes up to 45. A saturated saline solution required 30 and plant oil 35. The speed of dropping, pressure and position of the syringe or the lumen of the outlet, open or partially clogged with wax, did not have a significant influence on the number of drops. While the original research suggests that the division should be by 25 or more, we use the common notion. It is discouraged to use the drop as any standard unit.

The Hounsfield unit is a unit of X-ray attenuation used in evaluating CT scans. It is defined on an interval scale where air is -1000 HF, water is 0 HF and bone is +1000 HF. Any advice as to how this unit can be related to metric units of radiant intensity decremence is appreciated.

We have always pointed out that the homeopathic teaching takes potency not as equivalent to dilution and the C and X series would not equate to each other in the strictly numerical manner. Homeopathic potency includes the "agitation" (a vigorous shaking) that needs to occur in every step of the dilluting process. Therefore as of April 2010, the homeopathic units are declared "arbitrary units", that is, they are no longer convertible. Therefore, also, we discontinue defining them using the dilution functions. The dilution functions sometimes cause truly astronomical values, leading to overflow conditions, e.g. in such potencies as 30 C or 100 X or 10 M, which do actually occur in homeopathics that are on the market. The previous units continue to exist as "retired", but their symbols now have a prime (apostrophe) in them.

**§45 chemical and biochemical units** {#para-45} 

* 1. Units used mainly in chemical and biochemical laboratories are defined in [Table 19](#chemical).  
* 2. The meaning of the columns is declared in [§43](#para-43).  
* 3. The function pair denoted "`ph(1 mol/l)`" is defined as _f_pH(_x_) = - lg _x_ to convert from moles per liter to the pH value, and _f_pH-1(_x_) = 10-_x_ to convert from the pH value back to moles per liter.

#### Table 19: Units used in Chemical and Biomedical Laboratories {#chemical}

| name | kind of quantity | print | c/s | c/i | M   | definition value | definition unit |
| --- | --- | --- | --- | --- | --- | --- | --- |
| equivalents | amount of substance | eq  | `eq` | `EQ` | yes | 1   | `mol` |
| osmole | amount of substance (dissolved particles) | osm | `osm` | `OSM` | yes | 1   | `mol` |
| pH  | acidity | pH  | `[pH]` | `[PH]` | no  | •   | `pH(1 mol/l)` |
| gram percent | mass concentration | g%  | `g%` | `G%` | yes | 1   | `g/dl` |
| Svedberg unit | sedimentation coefficient | S   | `[S]` | `[S]` | no  | 1   | `10*-13.s` |
| high power field | view area in microscope | HPF | `[HPF]` | `[HPF]` | no  | 1   | `1` |
| low power field | view area in microscope | LPF | `[LPF]` | `[LPF]` | no  | 100 | `1` |
| katal | catalytic activity | kat | `kat` | `KAT` | yes | 1   | `mol/s` |
| Unit | catalytic activity | U   | `U` | `U` | yes | 1   | `umol/min` |
| international unit | arbitrary | IU  | `[iU]` | `[IU]` | yes | •   | •   |
| international unit | arbitrary | i.U. | `[IU]` | `[IU]` | yes | •   | •   |
| arbitrary unit | arbitrary | arb. U | `[arb'U]` | `[ARB'U]` | no  | •   | •   |
| United States Pharmacopeia unit | arbitrary | U.S.P. | `[USP'U]` | `[USP'U]` | no  | •   | •   |
| GPL unit | biologic activity of anticardiolipin IgG |     | `[GPL'U]` | `[GPL'U]` | no  | •   | •   |
| MPL unit | biologic activity of anticardiolipin IgM |     | `[MPL'U]` | `[MPL'U]` | no  | •   | •   |
| APL unit | biologic activity of anticardiolipin IgA |     | `[APL'U]` | `[APL'U]` | no  | •   | •   |
| Bethesda unit | biologic activity of factor VIII inhibitor |     | `[beth'U]` | `[BETH'U]` | no  | •   | •   |
| anti factor Xa unit | biologic activity of factor Xa inhibitor (heparin) |     | `[anti'Xa'U]` | `[ANTI'XA'U]` | no  | •   | •   |
| Todd unit | biologic activity antistreptolysin O |     | `[todd'U]` | `[TODD'U]` | no  | •   | •   |
| Dye unit | biologic activity of amylase |     | `[dye'U]` | `[DYE'U]` | no  | •   | •   |
| Somogyi unit | biologic activity of amylase |     | `[smgy'U]` | `[SMGY'U]` | no  | •   | •   |
| Bodansky unit | biologic activity of phosphatase |     | `[bdsk'U]` | `[BDSK'U]` | no  | •   | •   |
| King-Armstrong unit | biologic activity of phosphatase |     | `[ka'U]` | `[KA'U]` | no  | •   | •   |
| Kunkel unit | arbitrary biologic activity |     | `[knk'U]` | `[KNK'U]` | no  | •   | •   |
| Mac Lagan unit | arbitrary biologic activity |     | `[mclg'U]` | `[MCLG'U]` | no  | •   | •   |
| tuberculin unit | biologic activity of tuberculin |     | `[tb'U]` | `[TB'U]` | no  | •   | •   |
| 50% cell culture infectious dose | biologic activity (infectivity) of an infectious agent preparation | CCID50 | `[CCID_50]` | `[CCID_50]` | no  | •   | •   |
| 50% tissue culture infectious dose | biologic activity (infectivity) of an infectious agent preparation | TCID50 | `[TCID_50]` | `[TCID_50]` | no  | •   | •   |
| 50% embryo infectious dose | biologic activity (infectivity) of an infectious agent preparation | EID50 | `[EID_50]` | `[EID_50]` | no  | •   | •   |
| plaque forming units | amount of an infectious agent | PFU | `[PFU]` | `[PFU]` | no  | •   | •   |
| focus forming units | amount of an infectious agent | FFU | `[FFU]` | `[FFU]` | no  | •   | •   |
| colony forming units | amount of a proliferating organism | CFU | `[CFU]` | `[CFU]` | no  | •   | •   |
| index of reactivity | amount of an allergen calibrated through in-vivo testing using the Stallergenes® method | IR  | `[IR]` | `[IR]` | no  | •   | •   |
| bioequivalent allergen unit | amount of an allergen calibrated through in-vivo testing based on the ID50EAL method of (intradermal dilution for 50mm sum of erythema diameters | BAU | `[BAU]` | `[BAU]` | no  | •   | •   |
| allergen unit | procedure defined amount of an allergen using some reference standard | AU  | `[AU]` | `[AU]` | no  | •   | •   |
| allergen unit for Ambrosia artemisiifolia | procedure defined amount of the major allergen of ragweed | Amb a 1 U | `[Amb'a'1'U]` | `[AMB'A'1'U]` | no  | •   | •   |
| protein nitrogen unit | procedure defined amount of a protein substance | PNU | `[PNU]` | `[PNU]` | no  | •   | •   |
| Limit of flocculation | procedure defined amount of an antigen substance | Lf  | `[Lf]` | `[LF]` | no  | •   | •   |
| D-antigen unit | procedure defined amount of a poliomyelitis d-antigen substance |     | `[D'ag'U]` | `[D'AG'U]` | no  | •   | •   |
| fibrinogen equivalent unit | amount of fibrinogen broken down into the measured d-dimers |     | `[FEU]` | `[FEU]` | no  | •   | •   |
| ELISA unit | arbitrary ELISA unit |     | `[ELU]` | `[ELU]` | no  | •   | •   |
| Ehrlich unit | Ehrlich unit |     | `[EU]` | `[EU]` | no  | •   | •   |

The amount of electrolytes (including acids and bases) is often reported as _equivalents_ instead of amount of substance. This habit originates in the measuring technique of titration. _The Unified Code for Units of Measure_ does not endorse using equivalents. We rather recommend to calculate the proper amount of substance after titration, so that 1 eq of Na+ ions is 1 mol, but 1 eq of Ca++ ions is 0.5 mol. The problem with equivalents is that the measurement results are difficult to compare because their magnitude depends on the degree of ionization of the substance. That is to say, the meaning of equivalents depend not only on the substance, but also on the state that the substance is in. For example, in iron we have to distinguish Fe2+ from Fe3+, so that no one can be sure how much 1 eq of iron really is.

Degrees of acidity are normally measured as "the pH value" that is the negative decadic logarithmus of the concentration of free protons (or hydronium ions) expressed in 1 mol/l. Usually the pH value is considered a dimensionless quantity. With the semantics of special units ([§§21ff](#para-21)). _The Unified Code for Units of Measure_ can link the pH value tighter to the system of proper units. Thus "`[pH]`" is defined as a unit symbol with the corresponding unit 1 mol/l. This allows conversions between pH and concentrations, and---because _The Unified Code for Units of Measure_ identifies the mole with the Avogadro number---can be converted to an absolute number of protons: for example, pH 7.4 converts instantly to 0.04 μmol/l and approximately 23975 protons per picoliter.

The unit osmol as the amount of dissolved particles is to be used with caution because it interferes with "osmolar" which is the amount of dissolved particles per liter.

The gram-percent (g%) is a metric unit that has the same origin as %vol. Originally it was a dimensionless quantity expressing a ratio of two masses and thus equal to 1/100 g/g. Because water is the most important solvent in biochemistry and 1 g of a solution in water has a volume of approximately 1 ml, the meaning of the unit 1 g% drifted towards 1/100 g/ml and farther off to 1 g/dl. That way, the unit 1 g% regained a proper dimension (mass concentration, _M_/_L_3). Most often it is used as 1 mg% = 1 mg/dl but all other SI prefixes are possible.

The Svedberg unit S is used to classify macromolecules (e.g., ribosomes) in different phases of a centrifugate.

The units "high power field" (HPF) and "low power field" (LPF) are used in microscopic analysis mostly of urine sediments. These units are used in semi-quantitative estimations of the abundance of things like crystals, bacteria or red and white blood cells. The number of the objects of interest is counted in one view field in the microscope with a 10 times (low) or 100 times (high) magnifying objective lens and then reported as the number per LPF or per HPF respectively. Obviously the number of objects seen depends on the way the slide is prepared: the amount of emulgate dropped, its initial dilution, and the way the drop is smeared. These preparations of the slides are usually carried out with great routine but little exactness, hence LPF and HPF can hardly relate to any exact and meaningful volume.

The best we could do is to define LPF and HPF as areas of the viewed field. However, the area of the field varies with the kind of eyepiece used in the microscope. The so called "field number" of the eyepiece, i.e., the diameter of the view area is typically between 18 mm and 25 mm which is divided by the magnification of the objective lense to yield the actual field diameter _d_. Because the area _A_ = π d2, the LPF can be anywhere between 2.5 mm^2 and 5 mm^2 and the HPF between 0.025 mm^2 and 0.05 mm^2. Because of this inexactness, we define LPF and HPF as dimensionless quantities with magnitudes that reflect the ratio of the view areas, i.e. 100:1. This allows at least to convert between numbers per LPF and per HPF and vice versa.

The unit "U" of enzymatic activity was defined in 1964 by the _International Union of Biochemistry_ as the catalytic activity that catalyzes the transformation of 1 μmol of the substrate per minute. This unit is defined so that normal biological enzyme activities are in the range of 1 U-100 U. This unit could not be adopted by the CGPM because it violates the style rules of the SI, i.e. "unit" is a very indistinctive word, "U" is a capital letter, and the definition is not coherent with the SI.

An SI-coherent unit katal 1 kat = 1 mol/s, had been proposed for adoption into the SI over 30 years ago and was finally adopted by the CGPM in 1999. However, perhaps because the unit katal is 7 orders of magnitudes greater than normal catalytic activities, in practice the katal has not gained much in popularity over the unit "U".

In its 1999 decision to add the katal to the SI, the CGPM explicitly "recommends that when the katal is used, the measurand be specified by reference to the measurement procedure; the measurement procedure must identify the indicator reaction." The general problem with catalytic activities is that these heavily depend not only on the substance but on many side-conditions, such as temperature, acidity of the solution, presence or absence of cofactors, inhibitors or activators, and the amount of substrate. Particularly a catalytic activity measured _in vitro_ says little about the activity _in vivo_. Hence the use of katal alone without specifying exactly the measurement method, is not sufficient to improve comparability of the measurement of catalytic substances.

Because of the influence of the measurement method, results of biologic activity measurement cannot usually be converted. This is a particular problem with the many named arbitrary units that are still used. _The Unified Code for Units of Measure_ initially defined all arbitrary units as dimensionless. But since this leads to the false conclusion that all arbitrary units are the same, the _Unified Code for Units of Measure_ now accounts for arbitrary units using a special flag. When a unit is marked as arbitrary, it is isolated from all other units, and no result can be converted from and to that unit (See [§24](#para-24)).

The unit "TCID50" expresses the result of quantifying an infectious agent in tissue culture. It is a titer, expressing the highest dilution of the specimen which produces a cytopathic effect in 50% of the cell cultures or wells inoculated. [Sources: Clinical Microbiology Reviews, July 1998, Vol. 11(3), p. 533-554]

The unit "CCID50" expresses the result of quantifying an infectious agent in a cell culture. It is a titer, expressing the highest dilution of the specimen which produces a cytopathic effect in 50% of the cell cultures or wells inoculated. [Sources: Schmidt NJ. Cell culture procedures for diagnostic virology, p. 78-79. In Schmidt NJ, Emmons RW (ed.), _Diagnostic procedures for viral, rickettsial and chlamydial infections_, 5th ed. American Public Health Association, Inc., Washington, D.C.]

The unit "PFU" measures viral infectivity in a sensitive assay in cell culture where the titer is determined by counting the number of visible plaques developed following viral infection of a sensitive cell culture and results recorded as PFU/ml.

The unit "FFU" measures viral infectivity in a sensitive assay in cell culture, for example, using immunofocus or vital dyes technology. For example, the titer is determined by visualizing infected areas of a cell monolayer by probing with virus-specific antibodies and results are recorded as FFU/ml. \[Sources: _WHO expert committee on biological standardization (55th Edition)._ WHO Technical Report #932;\]

The unit "BAU" measures amount of an allergen based on an in-vivo calibrated test using the Intradermal Dilution for 50mm sum of Erythema Diameters (ID50EAL) Method. \[Source: _Turkeltaub PC. Biological Standardization based on Quantitative Skin Testing - The 1D50 EAL Method. Arbeiten aus dem Paul-Ehrlich-Institut, dem Georg-Speyer-Haus und dem Ferdinand-Blum-Institut, Band 80 Gustav Fischer Verlag' Stuttgart, New York. 1987_\]

EDITORIAL NOTE: This method needs to be further investigated to determine a quantitative model which relates that would relate 1 BAU with a standardized amount of substance of the standardized allergenic protein. The situation is not unlike the titer and is not worse than for many of the arbitrary units listed already. In a future revision a stronger formalized metrologic model will be added to this specification.

The unit "AU" (for allergen unit) is for the amount of an allergen based some procedure defined and allergen specific reference standard. Note, do not confuse with astronomical unit, distinguish `[AU]` from `AU`

The unit "IR" has been defined to measure the allergenicity of an allergen extract. The allergen extract contains 100 IR/ml when, on a skin prick-test using a Stallerpoint®, it induces a wheal diameter of 7 mm in 30 patients sensitized to this allergen, (geometric mean). The cutaneous reactivity of these patients is simultaneously demonstrated by a positive skin prick-test to either 9 % codeine phosphate or 10 mg/ml histamine. The IR unit of Stallergenes is not comparable to the units used by other allergen manufacturers.

EDITORIAL NOTE: Should more manufacturer specific units come up in the future, we will include a manufacturer abbreviation in the unit symbol.

The unit "Amb a 1 U" is an arbitrary unit for the amount of Amb a 1, a 38 kD glycoprotein that is the major allergen in short ragweed (Ambrosia artemisiifolia) pollen allergen extracts. The amount of Amb a 1 units are determined by an in-vitro comparison of a test short ragweed extract to a FDA CBER Amb a 1 reference standard. Amb a 1 is the up-to-date term for the short ragweed pollen allergen that was originally described as Antigen E. They are synonyms. Although Antigen E is no longer used in the scientific literature, its meaning is unambiguous. The manufacturers are still licensed to use Antigen E as the designation. Therefore, Amb a 1 U = AgE U. There is an empiric relationship between Amb a 1 U and BAU (350 Amb a 1 U/mL = 100,000 BAU/mL). It was based on studies done decades ago on 15 study subjects. FDA's CBER considered mandating a conversion to BAU/mL in the labeling of short ragweed pollen products, based on AgE content, but this was never implemented. CBER provides two US standard reagents to manufacturers for their determination of Amb a 1 content, a reference standard and a reference serum. The assay used is a radial immunodiffusion assay (RID). Solid references discussing the relationship between Antigen E U/mL/Amb a 1 U/mL and micrograms of Antigen E U/mL/Amb a 1/mL are being researched.

EDITORIAL NOTE: The University of Texas' Structural Database of Allergenic Proteins (SDAP) contains close to 1000 allergens, isoallergens. Comparing the prospect of thousands of such special units for every allergen, one begins to appreciate even the metrologically complex BAU unit.

The unit "PNU" is defined as follows: 1 PNU/ml is equivalent to 1 x 10-5 mg of nitrogen determined to be in the material precipitated from 1 ml of allergenic extract by phosphotungstic acid (micro-Kjeldahl method). Typically, 1 mg of protein nitrogen equals 100,000 PNU. The unit "PNU" is an old protein unit unrelated to SI units. Several hundred products, from several manufacturers, are labeled in PNUs, and a switch to SI units for protein content is impractical.

The unit "Lf" is called the "Limit of Flocculation" or "limes flocculationis". It is based on an antigen-antibody precipitation reaction and used for the quantification of the antigenic content of tetanus and diphteria toxin and toxoid. The limes flocculationis is the smallest amount of antigen that when mixed with one unit (Ramon) of antitoxin (antibody), produces the most rapid floccules in the flocculation test. For a purified crystalline tetanus or diphteria toxin 1 Lf is equivalent to ~ 2 μg of protein. For tetanus and diphtheria toxoids, antigenic purity is defined and controlled by Lf units per mg of protein nitrogen.

Many sources describe the unit of antitoxin as "international unit" (IU), however, this is no longer correct. It was correct for the first international standard for antitoxin, established in 1920s. It had an arbitrary unit defined as IU for in vivo antitoxic activity and that unit was also used for establishing Lf units of toxins and toxoids, that is why this standard had a ratio of 1 between flocculating activity (Lf) and antitoxic activity (IU). When WHO replaced that standard in 1970s, the second international standard related to Lf by a factor of 1.4 instead of 1. Ultimately, WHO decided to move to the toxoid standards and calibrated tetanus toxoid for flocculation using Lf unit (not IU). With the implementation of WHO standards for flocculation as tetanus and diphtheria toxoids, antitoxin standards were discontinued by the WHO. [Source: _Lyng J. Quantitative Estimation of Diphtheria and Tetanus Toxoids - 4 - Toxoids as International Reference Materials Defining Lf-units for Diphtheria and Tetanus Toxoids. Biologicals (1990) 18, 11-17._ Also on the definition of the IU for antitoxin: _Spaun J, Lyng J. Replacement of the International Standard for Tetanus Antitoxin and the Use of the Standard in the Flocculation Test. Bull. Wid Hith Org. 1970, 42, 523-534._`http://www.ncbi.nlm.nih.gov/pmc/articles/PMC2427455` and personal communication with FDA CBER representatives.]

**§46 levels** {#para-46} 

* 1. Pseudo-units defined to express logarithms of ratios between two quantities of the same kind are defined in [Table 20](#levels).  
* 2. The meaning of the columns is declared in [§43](#para-43).  
* 3. The function pairs denoted "`ln`" "`lg`" and "`2lg`" are defined as the natural logarithm, the decadic logarithm, and the decadic logarithm times two with their respective inverse functions.

#### Table 20: Levels {#levels}

| name | kind of quantity | print | c/s | c/i | M   | definition value | definition unit |
| --- | --- | --- | --- | --- | --- | --- | --- |
| neper | level | Np  | `Np` | `NEP` | yes | •   | `ln(1 1)` |
| bel | level | B   | `B` | `B` | yes | •   | `lg(1 1)` |
| bel sound pressure | pressure level | B(SPL) | `B[SPL]` | `B[SPL]` | yes | •   | `2lg(2 10*-5.Pa)` |
| bel volt | electric potential level | B(V) | `B[V]` | `B[V]` | yes | •   | `2lg(1 V)` |
| bel millivolt | electric potential level | B(mV) | `B[mV]` | `B[MV]` | yes | •   | `2lg(1 mV)` |
| bel microvolt | electric potential level | B(μV) | `B[uV]` | `B[UV]` | yes | •   | `2lg(1 uV)` |
| bel 10 nanovolt | electric potential level | B(10 nV) | `B[10.nV]` | `B[10.NV]` | yes | •   | `2lg(10 nV)` |
| bel watt | power level | B(W) | `B[W]` | `B[W]` | yes | •   | `lg(1 W)` |
| bel kilowatt | power level | B(kW) | `B[kW]` | `B[KW]` | yes | •   | `lg(1 kW)` |

These units are "pseudo-units" because of their standardized definition as being logarithms of a ratio of two measurements with the same kind-of-quantity: first, the units cancel out, and second, the logarithm does not produce a new unit. These units were defined as "metric" because they are used as such, although a multiplication operation is not defined on these quantities. Multiplication of the measurement value with a scalar _r_ is equivalent to raising the original ratio to the _r_-th power.

According to NIST, the neper is used as the ratio level of field quantities and the bel is used for the level of power quantities. The factor 2 comes into play when field quantities (like electric potential) are expressed in decibel. The specialized bel-units B(V), B(mV), B(W), etc. are defined as the level of the measured quantity with reference quantities 1 V, 1 mV, and 1 W respectively. [NIST Sp. Pub. 811, 1995 Edition]

Given the sound pressure level expressed in dB(SPL) it is feasible to define dB(A) for the A scale of loudness. Similar units such as phon and sone could be defined as well if a good approximation for the respective characteristic functions are available. Any advice is welcome.

**§47 miscellaneous units** {#para-47} 

* 1. Not otherwise classified units are defined in [Table 21](#misc).  
* 2. The meaning of the columns is declared in [§30](#para-30).  
* 3. Only the columns titled "c/s," "c/i," "M," "value," and "definition" are normative. Full name and print symbol are either not standardized or standardized by other bodies and are out of scope of _The Unified Code for Units of Measure_.  
* 4. The function pair denoted "`sqrt`" is defined as the square root with its respective inverse function, the square.

#### Table 21: Miscellaneous Units {#misc}

| name | kind of quantity | print | c/s | c/i | M   | definition value | definition unit |
| --- | --- | --- | --- | --- | --- | --- | --- |
| stere | volume | st  | `st` | `STR` | yes | 1   | `m3` |
| Ångström | length | Å   | `Ao` | `AO` | no  | 0.1 | `nm` |
| barn | action area | b   | `b` | `BRN` | no  | 100 | `fm2` |
| technical atmosphere | pressure | at  | `att` | `ATT` | no  | 1   | `kgf/cm2` |
| mho | electric conductance | mho | `mho` | `MHO` | yes | 1   | `S` |
| pound per square inch | pressure | psi | `[psi]` | `[PSI]` | no  | 1   | `[lbf_av]/[in_i]2` |
| circle | plane angle | circ | `circ` | `CIRC` | no  | 2   | `[pi].rad` |
| sphere | solid angle | sph | `sph` | `SPH` | no  | 4   | `[pi].sr` |
| metric carat | mass | ctm | `[car_m]` | `[CAR_M]` | no  | 0.2 | `g` |
| carat of gold alloys | mass fraction | ctAu | `[car_Au]` | `[CAR_AU]` | no  | 1   | `/24` |
| Smoot | length |     | `[smoot]` | `[SMOOT]` | no  | 67  | `[in_i]` |
| meter per square seconds per square root of hertz | amplitude spectral density |     | `[m/s2/Hz^(1/2)]` | `[M/S2/HZ^(1/2)]` | no  | •   | `sqrt(1 m2/s4/Hz)` |
| Nephelometric Turbidity Unit | turbidity |     | `[NTU]` | `[NTU]` | no  | 1   | `1` |
| Formazin Nephelometric Unit | turbidity |     | `[FNU]` | `[FNU]` | no  | 1   | `1` |

Although called "metric carat," the carat really is a customary unit, still used for precious gems. The word carat comes from Greek κερατίκον (small horn) that originally was the horn-shaped grain of a locust-tree species in the pea family, hence the carat grain is about three barley grain that the other English systems of weights are based on. The arab carat was 1/24 of an ounce, the Imperial carat (1877) was 205.3 mg or 3.168 grain. In other European cities, the carat was 205.8 mg (Hamburg, Lisboa) but there were great variations from 188.5 mg (Bologna) to 213.5 mg (Torino). Due to these variations no customary carat has gained importance today aside from the "metric carat" defined as 200 mg exactly. [_All About Carats_ URL: `http://www.channel1.com/users/scales/carat-def.htm`]

The "Mark" was a mass unit for precious metals (Köln 234 g, Paris 245 g, Wien 277 g). A mark of gold was subdivided into 24 "karat," a mark of silver into 16 "lot." This led to the other use of the unit "carat" to mean 1/24 in measuring the finesse of pure gold in an alloy. For example, an 8 carat gold alloy contains 8 parts of gold on 16 parts of silver = 8/24 = 1/3, or 333 per mille. This carat is spelled "karat" in the U.S. while other countries do not use different spellings.

The unit "`[m/s2/Hz^(1/2)]`" is defined as a special unit to represent the odd fractional exponent of the second obtaining for the unit of the amplitude spectral density (ASD). It is defined based on the unit for the power spectral density (PSD), that is 1 (m/s2)2/Hz or 1 m2 · s-3. Since the two measurements are directly comparable, PSD = ASD2.