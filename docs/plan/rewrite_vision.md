# Rewrite

## This document

this document exists for my own sake, i need some space to get my ideas down and my ultimate goal for the rewrite. It's more or less a planning document, but i dont do formalities

## Goal

The architecture and general organization here isnt great, theres also no documentation. I want to:
- make the code more **extensible**, implementing *dependency inversion* where processor variants can be constructed by injecting an architecture trait object
- simplify usage by implementing **interior mutability for memory objects**. Probably just a refcell but that would need to be enforced for any project using this (which realistically is only my own, so who cares). 
**Therefore** cycle(&mut mem) becomes cycle() which is much nicer
- **Better project structure**, although as of yet im not entirely sure how. I suppose itll come about after some further planning
- **Better test harness**, more organized and maybe some report generation (tests could pass for functionality but not cycle accuracy). Probably will just have claude writing tests.
- **Have a Dedicated Instruction Register**
- **Rework the flag register** its fine? But i might keep it as a u8 and have some other helper to interpret them as bools, essentially backwards from how it is. Conforms things nicer.
- **Better mOps** I want micro ops to be more accurate to the actual 6502 m ops, as well as that, 
- **Move full decode logic** to the new, aforementioned architecture objects
- **Actual interrupt and ADC** the adc thing is part of the architecture, it might make sense for interrupts to be part of architecture (ie nor IRQ or NMI for 6507), it is a part of the *actual* cpu architecture, but so then is everything really, lines need to be drawn but it might not work cleanly with the model