
# ITGConvert

A Rust-based tool to convert ITGMania scores to (approximate) DDR scores.

This is primarily intended for scores for usage on services like [LIFE4](https://life4ddr.com/), estimating performance on a home setup before "the real thing" in an arcade.







## Warning

This produces very approximate scores. One-to-one conversions are not possible, thanks to the different timing windows used in the different games. The timing windows [can be seen here](https://itgwiki.dominick.cc/en/software/stepmania-judgements). ITGMania has stricter timing on Fantastic judgements than DDR's Marvelous/Perfect judgements, but looser timing on all other judgements. This means that for good scores, this will likely *underestimate* your score, while for lower scores, it will likely *overestimate* your score. 

Getting the precise conversion is impossible due to these timing window differences. If you have changed the timing window in ITGMania to anything other than the default setting, the disparity will likely be even larger, rendering the estimate even less accurate.
## Usage/Examples

Navigate to `C:\Users\$USERNAME\AppData\Roaming\ITGmania\Save\MachineProfile` and locate the `Stats.xml` file. Copy that file into the `itg_convert` folder. Then open a command prompt inside the `itg_convert` folder, and use `cargo run`.

The command prompt window should fill up with the converted scores and the corresponding letter grade.


## Acknowledgements

 - [ITG Wiki](https://itgwiki.dominick.cc/en/software/stepmania-judgements)
 - [Remywiki](https://remywiki.com/DanceDanceRevolution_SuperNOVA2_Scoring_System)
 - [ITGMania](https://www.itgmania.com/)

