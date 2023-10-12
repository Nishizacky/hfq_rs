
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
