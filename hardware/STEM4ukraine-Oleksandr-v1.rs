ha:cschem-sheet-v1 {
	ha:obj_indirect.1 {
		li:objects {
			ha:group.1 {
				uuid=SZfwSnO+wnLw0LmbMjwAAABZ;
				li:objects {
					ha:group.1 {
						uuid=SZfwSnO+wnLw0LmbMjwAAABa; loclib_name=led5;
						li:objects {
						}
						ha:attrib {
							footprint=LED5
							li:portmap {
								{C->pcb/pinnum=1}
								{A->pcb/pinnum=2}
							}
						}
					}
					ha:group.2 {
						uuid=SZfwSnO+wnLw0LmbMjwAAABs; loclib_name=led_led3;
						li:objects {
						}
						ha:attrib {
							footprint=LED3
							li:portmap {
								{A->pcb/pinnum=1}
								{C->pcb/pinnum=2}
							}
						}
					}
				}
				ha:attrib {
					ha:purpose = { value=devmap; prio=0; }
				}
			}
		}
	}
	ha:obj_direct.2 {
		uuid=3s0ePx27Ce5+YP4+xH0AAAAC;
		li:objects {
			ha:pen.sheet-decor { shape=round; size=125; color=#777777; font_height=3000; font_family=sans; }
			ha:pen.sheet-decor-fill { shape=round; size=125; color=#bbbbbb; font_height=3000; font_family=sans; }
			ha:pen.titlebox-frame { shape=round; size=250; color=#777777; font_height=0; }
			ha:pen.titlebox-fill { shape=round; size=250; color=#bbffbb; font_height=0; }
			ha:pen.titlebox-big { shape=round; size=250; color=#777777; font_height=3000; font_family=sans; }
			ha:pen.titlebox-small { shape=round; size=250; color=#777777; font_height=1500; font_family=sans; }
			ha:pen.wire { shape=round; size=250; color=#2222bb; font_height=3000; font_family=sans; }
			ha:pen.bus { shape=round; size=1500; color=#2222bb; font_height=3000; font_family=sans; }
			ha:pen.hub { shape=round; size=3000; color=#6666ff; font_height=3000; font_family=sans; }
			ha:pen.sym-decor { shape=round; size=125; color=#119911; font_height=3000; font_family=sans; }
			ha:pen.sym-decor-fill { shape=round; size=125; color=#99ff99; font_height=3000; font_family=sans; }
			ha:pen.sym-primary { shape=round; size=125; color=#119911; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.sym-secondary { shape=round; size=125; color=#33bb33; font_height=3000; font_family=sans; }
			ha:pen.term-decor { shape=round; size=250; color=#222222; font_height=3000; font_family=sans; }
			ha:pen.term-primary { shape=round; size=250; color=#222222; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.term-secondary { shape=round; size=250; color=#555555; font_height=3000; font_family=sans; }
			ha:pen.busterm-decor { shape=round; size=1500; color=#222222; font_height=3000; font_family=sans; }
			ha:pen.busterm-primary { shape=round; size=1500; color=#222222; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.busterm-secondary { shape=round; size=1500; color=#555555; font_height=3000; font_family=sans; }
			ha:pen.junction { shape=round; size=1000; color=#2222bb; font_height=3000; font_family=sans; }
			ha:group.24 {
				uuid=3s0ePx27Ce5+YP4+xH0AAAD4; src_uuid=3s0ePx27Ce5+YP4+xH0AAADp;
				x=48000; y=140000; rot=270.000000;
				li:objects {
					ha:arc.1 { cx=6000; cy=9000; r=6000; sang=0.000000; dang=-180.000000; stroke=sym-decor; }
					ha:group.2 {
						uuid=3s0ePx27Ce5+YP4+xH0AAAD5; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB4;
						x=12000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
							ha:spice/pinnum = { value=2; prio=31050; }
						}
					}
					ha:group.3 {
						uuid=3s0ePx27Ce5+YP4+xH0AAAD6; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB5;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
							ha:spice/pinnum = { value=1; prio=31050; }
						}
					}
					ha:line.4 { x1=0; y1=9000; x2=12000; y2=9000; stroke=sym-decor; }
					ha:line.5 { x1=4000; y1=0; x2=4000; y2=3343; stroke=sym-decor; }
					ha:line.6 { x1=8000; y1=0; x2=8000; y2=3343; stroke=sym-decor; }
					ha:text.7 { x1=0; y1=3000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					footprint=piezo_5mm_to_7.62mm.rf
					name=PIEZO
					role=symbol
				}
			}
			ha:group.29 {
				uuid=3s0ePx27Ce5+YP4+xH0AAAFH;
				x=6000; y=8000;
				li:objects {
					ha:line.3 { x1=72000; y1=136000; x2=72000; y2=152000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.33 {
				uuid=3s0ePx27Ce5+YP4+xH0AAAFI;
				x=-8000; y=0;
				li:objects {
					ha:line.1 { x1=56000; y1=124000; x2=56000; y2=128000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.37 {
				uuid=3s0ePx27Ce5+YP4+xH0AAAFO; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=48000; y=124000;
				li:objects {
					ha:group.1 {
						uuid=3s0ePx27Ce5+YP4+xH0AAAFP; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.39 {
				uuid=3s0ePx27Ce5+YP4+xH0AAAFQ; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=78000; y=144000;
				li:objects {
					ha:group.1 {
						uuid=3s0ePx27Ce5+YP4+xH0AAAFR; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.41 {
				uuid=3s0ePx27Ce5+YP4+xH0AAAFS;
				x=-4000; y=0;
				li:objects {
					ha:line.3 { x1=52000; y1=140000; x2=52000; y2=156000; stroke=wire; }
					ha:line.4 { x1=52000; y1=156000; x2=40000; y2=156000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.56 {
				uuid=3s0ePx27Ce5+YP4+xH0AAAIH; src_uuid=90oZeuw9Jw0A8ciTCrAAAAAH;
				x=100000; y=156000;
				li:objects {
					ha:text.1 { x1=-8000; y1=-4000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.2 {
						li:outline {
							ha:line { x1=0; y1=0; x2=0; y2=20000; }
							ha:line { x1=0; y1=20000; x2=16000; y2=20000; }
							ha:line { x1=16000; y1=20000; x2=16000; y2=0; }
							ha:line { x1=16000; y1=0; x2=0; y2=0; }
						}
						stroke=sym-decor;
					}
					ha:group.3 {
						uuid=3s0ePx27Ce5+YP4+xH0AAAII; src_uuid=90oZeuw9Jw0A8ciTCrAAAAAB;
						x=0; y=16000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=MISO
							pinnum=1
							role=terminal
						}
					}
					ha:group.4 {
						uuid=3s0ePx27Ce5+YP4+xH0AAAIJ; src_uuid=90oZeuw9Jw0A8ciTCrAAAAAC;
						x=0; y=12000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=SCK
							pinnum=3
							role=terminal
						}
					}
					ha:group.5 {
						uuid=3s0ePx27Ce5+YP4+xH0AAAIK; src_uuid=90oZeuw9Jw0A8ciTCrAAAAAD;
						x=0; y=8000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=MOSI
							pinnum=4
							role=terminal
						}
					}
					ha:group.6 {
						uuid=3s0ePx27Ce5+YP4+xH0AAAIL; src_uuid=90oZeuw9Jw0A8ciTCrAAAAAE;
						x=0; y=4000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=RST
							pinnum=5
							role=terminal
						}
					}
					ha:group.7 {
						uuid=3s0ePx27Ce5+YP4+xH0AAAIM; src_uuid=90oZeuw9Jw0A8ciTCrAAAAAF;
						x=8000; y=20000; rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=Vcc
							pinnum=2
							role=terminal
						}
					}
					ha:group.8 {
						uuid=3s0ePx27Ce5+YP4+xH0AAAIN; src_uuid=90oZeuw9Jw0A8ciTCrAAAAAG;
						x=8000; y=0; rot=-90.000000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=GND
							pinnum=6
							role=terminal
						}
					}
					ha:text.9 { x1=-8000; y1=-8000; dyntext=1; stroke=sym-secondary; text=%../A.device%; floater=1; }
				}
				ha:attrib {
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-symbol-generator=boxsym-rnd
					device=ISP6pin
					footprint={connector(2,3,sequence=pivot,silkmark=external)}
					name=ISP
					role=symbol
				}
			}
			ha:group.92 {
				uuid=3s0ePx27Ce5+YP4+xH0AAALf;
				x=-12000; y=0;
				li:objects {
					ha:line.1 { x1=108000; y1=168000; x2=100000; y2=168000; stroke=wire; }
					ha:text.2 { x1=100000; y1=168000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=SCK
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.94 {
				uuid=3s0ePx27Ce5+YP4+xH0AAALg;
				x=-12000; y=0;
				li:objects {
					ha:line.1 { x1=100000; y1=164000; x2=108000; y2=164000; stroke=wire; }
					ha:text.2 { x1=100000; y1=164000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=MOSI
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.96 {
				uuid=3s0ePx27Ce5+YP4+xH0AAALh;
				x=-12000; y=0;
				li:objects {
					ha:line.1 { x1=100000; y1=160000; x2=108000; y2=160000; stroke=wire; }
					ha:text.2 { x1=100000; y1=160000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=RST
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.98 {
				uuid=3s0ePx27Ce5+YP4+xH0AAALi;
				x=-12000; y=0;
				li:objects {
					ha:line.1 { x1=100000; y1=172000; x2=108000; y2=172000; stroke=wire; }
					ha:text.2 { x1=100000; y1=172000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=MISO
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.107 {
				uuid=3s0ePx27Ce5+YP4+xH0AAALm; src_uuid=3s0ePx27Ce5+YP4+xH0AAAFJ;
				x=32000; y=24000;
				li:objects {
					ha:line.1 { x1=76000; y1=124000; x2=76000; y2=128000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.109 {
				uuid=3s0ePx27Ce5+YP4+xH0AAALn; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=108000; y=148000;
				li:objects {
					ha:group.1 {
						uuid=3s0ePx27Ce5+YP4+xH0AAALo; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.115 {
				uuid=3s0ePx27Ce5+YP4+xH0AAALz; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=108000; y=184000;
				li:objects {
					ha:group.1 {
						uuid=3s0ePx27Ce5+YP4+xH0AAAL0; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:group.116 {
				uuid=3s0ePx27Ce5+YP4+xH0AAAL1; src_uuid=3s0ePx27Ce5+YP4+xH0AAALv;
				x=32000; y=4000;
				li:objects {
					ha:line.1 { x1=76000; y1=180000; x2=76000; y2=176000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.119 {
				uuid=SZfwSnO+wnLw0LmbMjwAAAA0; src_uuid=/iiShtebwvwwWnNJ68YAAAAJ;
				x=16000; y=148000;
				li:objects {
					ha:text.1 { x1=0; y1=-8000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.2 {
						li:outline {
							ha:line { x1=0; y1=0; x2=0; y2=28000; }
							ha:line { x1=0; y1=28000; x2=16000; y2=28000; }
							ha:line { x1=16000; y1=28000; x2=16000; y2=0; }
							ha:line { x1=16000; y1=0; x2=0; y2=0; }
						}
						stroke=sym-decor;
					}
					ha:group.3 {
						uuid=SZfwSnO+wnLw0LmbMjwAAAA1; src_uuid=/iiShtebwvwwWnNJ68YAAAAB;
						x=16000; y=24000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=PB0
							pinnum=5
							role=terminal
						}
					}
					ha:group.4 {
						uuid=SZfwSnO+wnLw0LmbMjwAAAA2; src_uuid=/iiShtebwvwwWnNJ68YAAAAC;
						x=16000; y=20000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=PB1
							pinnum=6
							role=terminal
						}
					}
					ha:group.5 {
						uuid=SZfwSnO+wnLw0LmbMjwAAAA3; src_uuid=/iiShtebwvwwWnNJ68YAAAAD;
						x=16000; y=16000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=PB2
							pinnum=7
							role=terminal
						}
					}
					ha:group.6 {
						uuid=SZfwSnO+wnLw0LmbMjwAAAA4; src_uuid=/iiShtebwvwwWnNJ68YAAAAE;
						x=16000; y=12000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=PB3
							pinnum=2
							role=terminal
						}
					}
					ha:group.7 {
						uuid=SZfwSnO+wnLw0LmbMjwAAAA5; src_uuid=/iiShtebwvwwWnNJ68YAAAAF;
						x=16000; y=8000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=PB4
							pinnum=3
							role=terminal
						}
					}
					ha:group.8 {
						uuid=SZfwSnO+wnLw0LmbMjwAAAA6; src_uuid=/iiShtebwvwwWnNJ68YAAAAG;
						x=16000; y=4000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=PB5
							pinnum=1
							role=terminal
						}
					}
					ha:group.9 {
						uuid=SZfwSnO+wnLw0LmbMjwAAAA7; src_uuid=/iiShtebwvwwWnNJ68YAAAAH;
						x=8000; y=28000; rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=VCC
							pinnum=8
							role=terminal
						}
					}
					ha:group.10 {
						uuid=SZfwSnO+wnLw0LmbMjwAAAA8; src_uuid=/iiShtebwvwwWnNJ68YAAAAI;
						x=8000; y=0; rot=-90.000000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
							ha:text.3 { x1=-500; y1=-2000; mirx=1; dyntext=1; stroke=term-secondary; text=%../A.name%; }
						}
						ha:attrib {
							name=GND
							pinnum=4
							role=terminal
						}
					}
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Erich Heinzle
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-symbol-generator=boxsym-rnd
					footprint=dip(8)
					name=U1
					role=symbol
				}
			}
			ha:group.123 {
				uuid=SZfwSnO+wnLw0LmbMjwAAABD; src_uuid=3s0ePx27Ce5+YP4+xH0AAAFI;
				x=-32000; y=16000;
				li:objects {
					ha:line.1 { x1=56000; y1=124000; x2=56000; y2=128000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.125 {
				uuid=SZfwSnO+wnLw0LmbMjwAAABE; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=24000; y=140000;
				li:objects {
					ha:group.1 {
						uuid=SZfwSnO+wnLw0LmbMjwAAABF; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.127 {
				uuid=SZfwSnO+wnLw0LmbMjwAAABJ; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=24000; y=184000;
				li:objects {
					ha:group.1 {
						uuid=SZfwSnO+wnLw0LmbMjwAAABK; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:group.128 {
				uuid=SZfwSnO+wnLw0LmbMjwAAABL; src_uuid=3s0ePx27Ce5+YP4+xH0AAALv;
				x=-52000; y=4000;
				li:objects {
					ha:line.1 { x1=76000; y1=180000; x2=76000; y2=176000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.131 {
				uuid=SZfwSnO+wnLw0LmbMjwAAABM;
				x=0; y=8000;
				li:objects {
					ha:line.1 { x1=36000; y1=164000; x2=44000; y2=164000; stroke=wire; }
					ha:text.2 { x1=40000; y1=164000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=MOSI
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.133 {
				uuid=SZfwSnO+wnLw0LmbMjwAAABN;
				x=0; y=8000;
				li:objects {
					ha:line.1 { x1=44000; y1=160000; x2=36000; y2=160000; stroke=wire; }
					ha:text.2 { x1=40000; y1=160000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=MISO
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.135 {
				uuid=SZfwSnO+wnLw0LmbMjwAAABO;
				x=0; y=8000;
				li:objects {
					ha:line.1 { x1=36000; y1=156000; x2=44000; y2=156000; stroke=wire; }
					ha:text.2 { x1=40000; y1=156000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=SCK
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.144 {
				uuid=SZfwSnO+wnLw0LmbMjwAAABP;
				x=4000; y=8000;
				li:objects {
					ha:line.1 { x1=32000; y1=144000; x2=40000; y2=144000; stroke=wire; }
					ha:text.2 { x1=36000; y1=144000; dyntext=1; stroke=wire; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					name=RST
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.161 {
				li:conn {
					/2/131/1
					/2/119/3/1
				}
			}
			ha:connection.162 {
				li:conn {
					/2/133/1
					/2/119/4/1
				}
			}
			ha:connection.164 {
				li:conn {
					/2/135/1
					/2/119/5/1
				}
			}
			ha:connection.165 {
				li:conn {
					/2/144/1
					/2/119/8/1
				}
			}
			ha:connection.166 {
				li:conn {
					/2/123/1
					/2/119/10/1
				}
			}
			ha:connection.167 {
				li:conn {
					/2/123/1
					/2/125/1/1
				}
			}
			ha:connection.168 {
				li:conn {
					/2/128/1
					/2/119/9/1
				}
			}
			ha:connection.169 {
				li:conn {
					/2/128/1
					/2/127/1/1
				}
			}
			ha:connection.179 {
				li:conn {
					/2/41/4
					/2/119/7/1
				}
			}
			ha:connection.202 {
				li:conn {
					/2/33/1
					/2/24/2/1
				}
			}
			ha:connection.203 {
				li:conn {
					/2/37/1/1
					/2/33/1
				}
			}
			ha:connection.204 {
				li:conn {
					/2/41/3
					/2/24/3/1
				}
			}
			ha:group.233 {
				uuid=Rt1qKxs3HefkBG+oWGoAAAC7; src_uuid=Rt1qKxs3HefkBG+oWGoAAAC0;
				x=172000; y=172000; miry=1;
				li:objects {
					ha:text.1 { x1=0; y1=-6000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:group.2 {
						uuid=Rt1qKxs3HefkBG+oWGoAAAC8; src_uuid=Rt1qKxs3HefkBG+oWGoAAAC1;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:group.3 {
						uuid=Rt1qKxs3HefkBG+oWGoAAAC9; src_uuid=Rt1qKxs3HefkBG+oWGoAAAC2;
						x=0; y=4000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.4 {
						uuid=Rt1qKxs3HefkBG+oWGoAAAC+; src_uuid=Rt1qKxs3HefkBG+oWGoAAAC3;
						x=0; y=8000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=3
							role=terminal
						}
					}
					ha:group.5 {
						uuid=Rt1qKxs3HefkBG+oWGoAAAC/; src_uuid=Rt1qKxs3HefkBG+oWGoAAAC4;
						x=0; y=12000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=4
							role=terminal
						}
					}
					ha:group.6 {
						uuid=Rt1qKxs3HefkBG+oWGoAAADA; src_uuid=Rt1qKxs3HefkBG+oWGoAAAC5;
						x=0; y=16000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=5
							role=terminal
						}
					}
					ha:group.7 {
						uuid=Rt1qKxs3HefkBG+oWGoAAADB; src_uuid=Rt1qKxs3HefkBG+oWGoAAAC6;
						x=0; y=20000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							footprint=USB_B_180_degree_PTH.rf
							name=6
							role=terminal
						}
					}
					ha:polygon.8 {
						li:outline {
							ha:line { x1=0; y1=-2000; x2=0; y2=22000; }
							ha:line { x1=0; y1=22000; x2=4000; y2=22000; }
							ha:line { x1=4000; y1=22000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=0; y2=-2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					footprint=USB_B_180_degree_PTH_universal-v1.rf
					name=USB_B
					role=symbol
				}
			}
			ha:group.234 {
				uuid=Rt1qKxs3HefkBG+oWGoAAADF; src_uuid=3s0ePx27Ce5+YP4+xH0AAALv;
				x=88000; y=4000;
				li:objects {
					ha:line.1 { x1=76000; y1=180000; x2=76000; y2=168000; stroke=wire; }
					ha:line.4 { x1=76000; y1=168000; x2=76000; y2=168000; stroke=junction; }
					ha:line.5 { x1=44000; y1=168000; x2=80000; y2=168000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.235 {
				uuid=Rt1qKxs3HefkBG+oWGoAAADG; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=164000; y=184000;
				li:objects {
					ha:group.1 {
						uuid=Rt1qKxs3HefkBG+oWGoAAADH; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:group.242 {
				uuid=Rt1qKxs3HefkBG+oWGoAAADL; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=164000; y=148000;
				li:objects {
					ha:group.1 {
						uuid=Rt1qKxs3HefkBG+oWGoAAADM; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.244 {
				uuid=Rt1qKxs3HefkBG+oWGoAAADT; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAh;
				x=148000; y=172000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=Rt1qKxs3HefkBG+oWGoAAADU; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAi;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=Rt1qKxs3HefkBG+oWGoAAADV; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAj;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=5000; x2=9000; y2=-5000; stroke=sym-decor; }
					ha:line.6 { x1=11000; y1=5000; x2=11000; y2=-5000; stroke=sym-decor; }
					ha:line.7 { x1=4000; y1=0; x2=9000; y2=0; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=0; x2=16000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					footprint=C200.rf
					name=C1
					role=symbol
					ha:spice/prefix = { value=C; prio=31050; }
					value=100n
				}
			}
			ha:group.247 {
				uuid=Rt1qKxs3HefkBG+oWGoAAAEE; src_uuid=Rt1qKxs3HefkBG+oWGoAAAD/;
				x=66000; y=152000;
				li:objects {
					ha:text.1 { x1=-8000; y1=-4000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:group.2 {
						uuid=Rt1qKxs3HefkBG+oWGoAAAEF; src_uuid=Rt1qKxs3HefkBG+oWGoAAAEA;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:line.3 { x1=0; y1=0; x2=1200; y2=0; stroke=sym-decor; }
					ha:group.4 {
						uuid=Rt1qKxs3HefkBG+oWGoAAAEG; src_uuid=Rt1qKxs3HefkBG+oWGoAAAEB;
						x=8000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=4
							role=terminal
						}
					}
					ha:line.5 { x1=6800; y1=0; x2=8000; y2=0; stroke=sym-decor; }
					ha:arc.6 { cx=6400; cy=0; r=400; sang=0.000000; dang=360.000000; stroke=sym-decor; }
					ha:line.7 { x1=1200; y1=0; x2=5600; y2=3200; stroke=sym-decor; }
					ha:group.8 {
						uuid=Rt1qKxs3HefkBG+oWGoAAAEH; src_uuid=Rt1qKxs3HefkBG+oWGoAAAEC;
						x=0; y=8000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:line.9 { x1=0; y1=8000; x2=1200; y2=8000; stroke=sym-decor; }
					ha:group.10 {
						uuid=Rt1qKxs3HefkBG+oWGoAAAEI; src_uuid=Rt1qKxs3HefkBG+oWGoAAAED;
						x=8000; y=8000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=3
							role=terminal
						}
					}
					ha:line.11 { x1=6800; y1=8000; x2=8000; y2=8000; stroke=sym-decor; }
					ha:arc.12 { cx=6400; cy=8000; r=400; sang=0.000000; dang=360.000000; stroke=sym-decor; }
					ha:line.13 { x1=1200; y1=8000; x2=5600; y2=11200; stroke=sym-decor; }
					ha:line.14 { x1=4000; y1=2000; x2=4000; y2=6000; stroke=sym-decor; }
				}
				ha:attrib {
					footprint=TACT_6x6_4p
					name=SW1
					role=symbol
				}
			}
			ha:group.260 {
				uuid=Rt1qKxs3HefkBG+oWGoAAAEr; src_uuid=Rt1qKxs3HefkBG+oWGoAAAEo;
				x=128000; y=172000; mirx=1; miry=1;
				li:objects {
					ha:text.1 { x1=-5000; y1=-6000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:group.2 {
						uuid=Rt1qKxs3HefkBG+oWGoAAAEs; src_uuid=Rt1qKxs3HefkBG+oWGoAAAEp;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:group.3 {
						uuid=Rt1qKxs3HefkBG+oWGoAAAEt; src_uuid=Rt1qKxs3HefkBG+oWGoAAAEq;
						x=0; y=4000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:polygon.4 {
						li:outline {
							ha:line { x1=0; y1=-2000; x2=0; y2=6000; }
							ha:line { x1=0; y1=6000; x2=4000; y2=6000; }
							ha:line { x1=4000; y1=6000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=0; y2=-2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					footprint=connector(1,2)
					name=EXT_5V
					role=symbol
				}
			}
			ha:group.262 {
				uuid=Rt1qKxs3HefkBG+oWGoAAAEu;
				x=-24000; y=4000;
				li:objects {
					ha:line.1 { x1=156000; y1=164000; x2=156000; y2=148000; stroke=wire; }
					ha:line.2 { x1=192000; y1=156000; x2=188000; y2=156000; stroke=wire; }
					ha:line.3 { x1=188000; y1=156000; x2=188000; y2=144000; stroke=wire; }
					ha:line.4 { x1=188000; y1=148000; x2=188000; y2=148000; stroke=junction; }
					ha:line.5 { x1=188000; y1=152000; x2=192000; y2=152000; stroke=wire; }
					ha:line.6 { x1=188000; y1=152000; x2=188000; y2=152000; stroke=junction; }
					ha:line.8 { x1=156000; y1=148000; x2=192000; y2=148000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.276 {
				li:conn {
					/2/39/1/1
					/2/29/3
				}
			}
			ha:connection.278 {
				li:conn {
					/2/247/4/1
					/2/29/3
				}
			}
			ha:connection.280 {
				li:conn {
					/2/247/10/1
					/2/29/3
				}
			}
			ha:group.283 {
				uuid=REsy5/GoOXgCWMisbSAAAABn;
				li:objects {
					ha:line.1 { x1=36000; y1=160000; x2=62000; y2=160000; stroke=wire; }
					ha:line.2 { x1=62000; y1=160000; x2=62000; y2=152000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.284 {
				li:conn {
					/2/283/1
					/2/119/6/1
				}
			}
			ha:connection.285 {
				li:conn {
					/2/283/1
					/2/247/8/1
					/2/283/2
				}
			}
			ha:connection.292 {
				li:conn {
					/2/116/1
					/2/56/7/1
				}
			}
			ha:connection.310 {
				li:conn {
					/2/92/1
					/2/56/4/1
				}
			}
			ha:connection.311 {
				li:conn {
					/2/94/1
					/2/56/5/1
				}
			}
			ha:connection.312 {
				li:conn {
					/2/96/1
					/2/56/6/1
				}
			}
			ha:connection.313 {
				li:conn {
					/2/98/1
					/2/56/3/1
				}
			}
			ha:connection.314 {
				li:conn {
					/2/107/1
					/2/56/8/1
				}
			}
			ha:connection.315 {
				li:conn {
					/2/109/1/1
					/2/107/1
				}
			}
			ha:connection.316 {
				li:conn {
					/2/116/1
					/2/115/1/1
				}
			}
			ha:connection.318 {
				li:conn {
					/2/234/5
					/2/233/2/1
				}
			}
			ha:connection.319 {
				li:conn {
					/2/244/2/1
					/2/234/5
				}
			}
			ha:connection.320 {
				li:conn {
					/2/260/2/1
					/2/234/5
				}
			}
			ha:connection.322 {
				li:conn {
					/2/262/2
					/2/233/5/1
				}
			}
			ha:connection.323 {
				li:conn {
					/2/262/5
					/2/233/6/1
				}
			}
			ha:connection.324 {
				li:conn {
					/2/262/8
					/2/244/1/1
				}
			}
			ha:connection.325 {
				li:conn {
					/2/262/8
					/2/233/7/1
				}
			}
			ha:connection.326 {
				li:conn {
					/2/235/1/1
					/2/234/1
				}
			}
			ha:connection.327 {
				li:conn {
					/2/262/1
					/2/260/3/1
				}
			}
			ha:connection.328 {
				li:conn {
					/2/262/3
					/2/242/1/1
				}
			}
			ha:connection.329 {
				li:conn {
					/2/283/2
					/2/247/2/1
				}
			}
		}
		ha:attrib {
			drawing_min_height=200000
			drawing_min_width=287000
			maintainer=<maint. attr>
			page=<page attr>
			print_page=A/4
			title=<please set sheet title attribute>
		}
	}
  li:sch-rnd-conf-v1 {
   ha:overwrite {
    ha:editor {
     fullscreen = 0
     grids_idx = 1
     grid = 2.0480 mm
    }
   }
  }
}
