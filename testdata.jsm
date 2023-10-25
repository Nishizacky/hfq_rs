.include /home/nishizaki/myHFQenv/cktlib.jsm
# .include /home/nishizaki/hfq-optimizer/hfq_xor/cktconfig.py
*** ZZPSQ
.subckt ZZPSQ          1          2
***       Pin      Pout
L4                 3         2   0.200pH 
L3                 4         2   0.200pH 
L1                 1         5   1.400pH 
L0                 1         6   1.400pH 
B2                 5         7  jjmod area=20.00
RS2                5         7   0.20ohm
B1                 7         4  jjmod area=0.61
RS1                7         4  12.71ohm
B0                 6         3  jjmod area=0.61
RS0                6         3  12.71ohm
.ends
.subckt ZZPSQ_XOR_loop_0          1          2
***       Pin      Pout
L4                 3         2   0.200pH 
L3                 4         2   0.200pH 
L1                 1         5   1.400pH 
L0                 1         6   1.400pH 
B2                 5         7  jjmod_pi area=20
RS2                5         7   0.20ohm
B1                 7         4  jjmod area=0.6 #!zero_in0
RS1                7         4  12.71ohm
B0                 6         3  jjmod area=0.6 #!zero_in0
RS0                6         3  12.71ohm
.ends

.subckt ZZPSQ_XOR_loop_1          1          2
***       Pin      Pout
L4                 3         2   0.200pH 
L3                 4         2   0.200pH 
L1                 1         5   1.400pH 
L0                 1         6   1.400pH 
B2                 5         7  jjmod_pi area=20
RS2                5         7   0.20ohm
B1                 7         4  jjmod area=0.5 #!Zero_in1
RS1                7         4  12.71ohm
B0                 6         3  jjmod area=0.5 #!Zero_in1
RS0                6         3  12.71ohm
.ends

.subckt ZZPSQ_XOR_loop_2          1          2
***       Pin      Pout
L4                 3         2   0.200pH 
L3                 4         2   0.200pH 
L1                 1         5   1.400pH 
L0                 1         6   1.400pH 
B2                 5         7  jjmod_pi area=20
RS2                5         7   0.20ohm
B1                 7         4  jjmod area=0.7 *mark
RS1                7         4  12.71ohm
B0                 6         3  jjmod area=0.7 *mark
RS0                6         3  12.71ohm
.ends

.subckt ZZPSQ_XOR_loop_3          1          2
***       Pin      Pout
L4                 3         2   0.200pH 
L3                 4         2   0.200pH 
L1                 1         5   1.400pH 
L0                 1         6   1.400pH 
B2                 5         7  jjmod_pi area=20
RS2                5         7   0.20ohm
B1                 7         4  jjmod area=0.5 *mark
RS1                7         4  12.71ohm
B0                 6         3  jjmod area=0.5 *mark
RS0                6         3  12.71ohm
.ends

.subckt ZZPSQ_XOR_loop_4          1          2
***       Pin      Pout
L4                 3         2   0.200pH 
L3                 4         2   0.200pH 
L1                 1         5   1.400pH 
L0                 1         6   1.400pH 
B2                 5         7  jjmod_pi area=20
RS2                5         7   0.20ohm
B1                 7         4  jjmod area=0.6 *mark
RS1                7         4  12.71ohm
B0                 6         3  jjmod area=0.6 *mark
RS0                6         3  12.71ohm
.ends

.subckt ZZPSQ_XOR_loop_5          1          2
***       Pin      Pout
L4                 3         2   0.200pH 
L3                 4         2   0.200pH 
L1                 1         5   1.400pH 
L0                 1         6   1.400pH 
B2                 5         7  jjmod_pi area=20
RS2                5         7   0.20ohm
B1                 7         4  jjmod area=0.6 *mark
RS1                7         4  12.71ohm
B0                 6         3  jjmod area=0.6 *mark
RS0                6         3  12.71ohm
.ends

.subckt ZZPSQ_XOR_loop_6          1          2
***       Pin      Pout
L4                 3         2   0.200pH 
L3                 4         2   0.200pH 
L1                 1         5   1.400pH 
L0                 1         6   1.400pH 
B2                 5         7  jjmod_pi area=20
RS2                5         7   0.20ohm
B1                 7         4  jjmod area=0.5 *mark
RS1                7         4  12.71ohm
B0                 6         3  jjmod area=0.5 *mark
RS0                6         3  12.71ohm
.ends

.subckt ZZPSQ_XOR_escape_0          1          2
***       Pin      Pout
L4                 3         2   0.200pH 
L3                 4         2   0.200pH 
L1                 1         5   1.400pH 
L0                 1         6   1.400pH 
B2                 5         7  jjmod_pi area=20
RS2                5         7   0.20ohm
B1                 7         4  jjmod area=0.58 *mark
RS1                7         4  12.71ohm
B0                 6         3  jjmod area=0.58 *mark
RS0                6         3  12.71ohm
.ends

.subckt ZZPSQ_XOR_escape_1          1          2
***       Pin      Pout
L4                 3         2   0.200pH 
L3                 4         2   0.200pH 
L1                 1         5   1.400pH 
L0                 1         6   1.400pH 
B2                 5         7  jjmod_pi area=20
RS2                5         7   0.20ohm
B1                 7         4  jjmod area=0.65 *mark
RS1                7         4  12.71ohm
B0                 6         3  jjmod area=0.65 *mark
RS0                6         3  12.71ohm
.ends

.subckt ZZPSQ_XOR_escape_2          1          2
***       Pin      Pout
L4                 3         2   0.200pH 
L3                 4         2   0.200pH 
L1                 1         5   1.400pH 
L0                 1         6   1.400pH 
B2                 5         7  jjmod_pi area=20
RS2                5         7   0.20ohm
B1                 7         4  jjmod area=0.53 *mark
RS1                7         4  12.71ohm
B0                 6         3  jjmod area=0.53 *mark
RS0                6         3  12.71ohm
.ends

.subckt ZZPSQ_XOR_escape_3          1          2
***       Pin      Pout
L4                 3         2   0.200pH 
L3                 4         2   0.200pH 
L1                 1         5   1.400pH 
L0                 1         6   1.400pH 
B2                 5         7  jjmod_pi area=20
RS2                5         7   0.20ohm
B1                 7         4  jjmod area=0.9 *mark
RS1                7         4  12.71ohm
B0                 6         3  jjmod area=0.9 *mark
RS0                6         3  12.71ohm
.ends

*** hfq_xor
.subckt hfq_xor          8          9         10         11        16
***         a         b         c       clk         bias
R_Bias_A0         16        17   66ohm
R_Bias_B0         16        18   66ohm
R_Bias_CLK        16        19   67ohm
R_Bias_A1         16        20  154ohm
R_Bias_B1         16        21  154ohm

L_Bias_A0         17        28   0.211pH 
L_Bias_A1         20        32   0.351pH 
L_Bias_B0         18        30   0.211pH 
L_Bias_B1         21        25   0.361pH 
L_Bias_CLK        19        37   0.208pH 

L_A0               8        28   0.247pH 
L_A1             28        22   2.530pH 
L_A2             22        23   4.095pH 
L_A3             31        32   10pH 
L_A4             32        43   12pH 

L_B0              9        30   0.247pH 
L_B1             30        36   2.530pH 
L_B2             36        42   4.095pH 
L_B3             24        25   10pH 
L_B4             25        43   12pH 

L_CLK0           11        37   0.273pH 
L_CLK1           37        38   2.3pH 
L_CLK2           39        29   2.5pH 

L0                33         0   0.146pH 
LP3               35         0   0.127pH 
LP4               34         0   0.138pH 
LP5               44         0   0.307pH 
LP6               40         0   0.159pH 
LP7               26         0   0.153pH 
L14               41         0   0.146pH 

L_OUT0               45        29   1.8pH 
L_OUT1               27        10   2.213pH 
* L_OUT1               27        10   2.213pH 
L_OUT2               29        27   5.047pH 

X_B_IN              ZZPSQ_XOR_loop_0         36         35
X_B_LOOP            ZZPSQ_XOR_loop_1         42         34
X_LOOP_END          ZZPSQ_XOR_loop_2         29         44
X_OUT               ZZPSQ_XOR_loop_3         27         26
X_CLK_IN            ZZPSQ_XOR_loop_4         38         40
X_A_IN              ZZPSQ_XOR_loop_5         22         33
X_A_LOOP            ZZPSQ_XOR_loop_6         23         41
X_B_ESKP            ZZPSQ_XOR_escape_0       42         24
XLOOP_ESKP          ZZPSQ_XOR_escape_1       43         45
X_A_ESKP             ZZPSQ_XOR_escape_2       23         31
XCLK_ESKP           ZZPSQ_XOR_escape_3       46         38
XI132               pi_2shifter              46         39
.ends

*** top cell: test_hfq_xor
V1               54        0  PWL(0ps 0v  230.0ps 0v 231.0ps 50mv 331.0ps 50mv 332.0ps 0mv 430.0ps 0v 431.0ps 50mv 531.0ps 50mv 532.0ps 0mv 630.0ps 0v 631.0ps 50mv 731.0ps 50mv 732.0ps 0mv 830.0ps 0v 831.0ps 50mv 931.0ps 50mv 932.0ps 0mv)
XI27              dchfq         54         55         62
X_jtl_clk       hfq_jtl         55         17         62
XIclk01           hfq_jtl         17         160         62
XIclk02           hfq_jtl         160         161         62
XIclk03           hfq_jtl         161         162         62
XIclk04           hfq_jtl         162         163         62
XIclk05           hfq_jtl         163         164         62
XIclk06           hfq_jtl         164         23         62
XIclk07           hfq_jtl         23         52         62

V2                3         0  PWL(0ps 0v  550ps 0v 551ps 50mv 651ps 50mv 652ps 0mv 750ps 0v 751ps 50mv 851ps 50mv 852ps 0mv)
XI26            dchfq           3           5         62
X_jtl_a         hfq_jtl          5         57         62
XIa01            hfq_jtl         57         257         62
XIa02           hfq_jtl         257         258         62
XIa03           hfq_jtl         258         259         62
XIa04           hfq_jtl         259         260         62
XIa05           hfq_jtl         260         261         62
XIa06           hfq_jtl         261         58         62
XIa07           hfq_jtl         58         50         62

V3                4         0  PWL(0ps 0v  350ps 0v 351ps 50mv 451ps 50mv 452ps 0mv 750ps 0v 751ps 50mv 851ps 50mv 852ps 0mv)
XI28               dchfq          4         53         62
X_jtl_b          hfq_jtl         53         60         62
XIb01            hfq_jtl         60        361         62
XIb02            hfq_jtl        361        362         62
XIb03            hfq_jtl        362        363         62
XIb04            hfq_jtl        363        364         62
XIb05            hfq_jtl        364        365         62
XIb06            hfq_jtl        365         59         62
XIb07            hfq_jtl         59         49         62

X_XOR          hfq_xor         50         49         51         52         62
* X_xor          hfq_xor         0         0         0         0         0

XIs01          hfq_jtl         51         61         62
XIs02          hfq_jtl         61         460         62
XIs03          hfq_jtl         460         461         62
XIs04          hfq_jtl         461         462         62
XIs05          hfq_jtl         462         463         62
XIs06          hfq_jtl         463         464         62
XIs07          hfq_jtl         464         56         62
X_sink         hfq_sink        56         62
*  X_sink         hfq_sink         0         62

V0                62         0  PWL(0ps 0mv 100ps 1mV)

*** netlist file

*** jsim input file

*** jsim input file
.tran 0.1ps 980ps 0ps 0.1ps

* .print i L_CLK2|X_XOR
* .print i L_CLK1|X_XOR
* .print i L_A4|X_XOR
* .print i L_B4|X_XOR
* .print v L_CLK2|X_XOR
* .print v L_A4|X_XOR
* .print v L_B4|X_XOR

.print nodep 49|X_sink 48|X_sink
.print nodep 29|X_XOR
.print nodep 42|X_XOR
.print nodep 38|X_XOR
.print nodep 23|X_XOR
.print nodep 23|X_XOR 31|X_XOR
.print nodep 46|X_XOR 39|X_XOR
.print nodep 42|X_XOR 24|X_XOR
.plot nodep 43|X_XOR 45|X_XOR

.end


