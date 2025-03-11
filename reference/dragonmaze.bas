1 TEXT : CALL -936
2 PRINT "WELCOME TO THE DRAGON'S MAZE!"
3 PRINT "YOU MAY WATCH WHILE I BUILD A MAZE,"
4 PRINT "BUT WHEN IT'S COMPLETE, I'LL ERASE"
5 PRINT "THE PICTURE. THEN YOU'LL ONLY SEE THE WALLS AS YOU BUMP INTO THEM."
6 PRINT "TO MOVE, YOU HIT 'R' FOR RIGHT,"
7 PRINT "'L' FOR LEFT, 'U' FOR UP, AND"
8 PRINT "'D' FOR DOWN. DO NOT HIT RETURN!"
9 PRINT
10 PRINT "THE OBJECT IS FOR YOU (THE GREEN DOT"
11 PRINT "TO GET TO THE DOOR ON THE RIGHT SIDE"
12 PRINT "BEFORE THE DRAGON (THE RED DOT) EATS"
13 PRINT "YOU."
14 PRINT "BEWARE!!!!!!!!! SOMETIMES THE DRAGON"
15 PRINT "GETS REAL MAD, AND CLIMBS OVER A WALL."
16 PRINT "BUT MOST OF THE TIME, HE CAN'T GO OVER"
17 PRINT "AND HAS TO GO AROUND."

18 PRINT
19 PRINT "(HINT: YOU CAN OFTEN TELL WHERE A WALL"
20 PRINT "IS, EVEN BEFORE YOU CAN SEE IT, BY"
21 PRINT "THE FACT THAT THE DRAGON CAN'T GET"
22 PRINT "THROUGH IT!)"
23 PRINT
89 DIM A$(3)
90 PRINT "TYPE 'GO' TO BEGIN ";: INPUT A$
100 GR : COLOR=15
105 CALL -936: PRINT "DRAGON MAZE";: TAB(25): PRINT "GARY J. SHANNON"
110 FOR I=0 TO 39 STEP 3: VLIN 0,39 AT I: HLIN 0,39 AT I: NEXT I
120 COLOR=0
130 S=1000

999 REM --- MAZE GENERATION
1000 DIM M(169),T(169)
1001 FOR I=1 TO 169:T(I)=0: NEXT I
1010 FOR I=1 TO 169:M(I)=11: NEXT I

1030 X= RND (13)+1:Y= RND (13)+1:C=169
1035 IF C=1 THEN 1200

1040 R=0:D=0:L=0:U=0:K=X+13*(Y-1):M(K)=- ABS (M(K)):C=C-1
1050 IF X=13 THEN 1060:R=M(K+1)>0
1060 IF Y=13 THEN 1070:D=M(K+13)>0
1070 IF X=1 THEN 1080:L=M(K-1)>0
1080 IF Y=1 THEN 1090:U=M(K-13)>0
1090 Q=R+D+L+U
1100 IF (Q<3 AND RND (10)<2) OR Q=0 THEN 1170
1110 DR= RND (4)

1120 GOTO 1130+10*DR

1130 IF NOT R THEN 1110:M(K)=M(K)+1:X=X+1
1135 VLIN 3*Y-2,3*Y-1 AT 3*(X-1)
1136 GOTO 1035

1140 IF NOT D THEN 1110:M(K)=M(K)+10:Y=Y+1
1145 HLIN 3*X-2,3*X-1 AT 3*(Y-1)
1146 GOTO 1035

1150 IF NOT L THEN 1110:M(K-1)=M(K-1)-1:X=X-1
1155 VLIN 3*Y-2,3*Y-1 AT 3*X
1156 GOTO 1035

1160 IF NOT U THEN 1110:M(K-13)=M(K-13)-10:Y=Y-1
1165 HLIN 3*X-2,3*X-1 AT 3*Y: GOTO 1035

1170 X= RND (13)+1:Y= RND (13)+1
1180 IF M(X+13*(Y-1))>0 THEN 1170

1190 C=C+1: GOTO 1035
1200 GOSUB 5000: PRINT "THE MAZE IS READY"

1201 REM --- GAME SETUP: PLAYER POSITIONS & EXIT
1205 GR : COLOR=15
1210 VLIN 0,39 AT 0: VLIN 0,39 AT 39: HLIN 0,39 AT 0: HLIN 0,39 AT 39
1220 X=1:Y= RND (13)+1: COLOR=8: PLOT 3*X-2,3*Y-2
1225 HX=3*X-2:HY=3*Y-2
1230 WY= RND (13)+1
1240 COLOR=0: VLIN 3*WY-2,3*WY-1 AT 39
1250 SX=13:SY=WY
1260 QX=3*SX-2:QY=3*SY-2
1270 RD=1

1499 REM --- PLAYER MOVEMENT
1500 K= PEEK (-16384): IF K<128 THEN 1500
1510 POKE -16368,0
1515 QQ=K: GOSUB 7000:K=QQ
1516 IF SX=X AND SY=Y THEN 8000
1520 IF K= ASC("R") THEN 2000
1530 IF K= ASC("L") THEN 2500
1540 IF K= ASC("U") THEN 3000
1550 IF K= ASC("D") THEN 3500
1560 GOSUB 5000: GOTO 1500
2000 DX=1:DY=0
2010 IF M(X+13*(Y-1)) MOD 10 THEN 4000
2020 FX=3*X-2:FY=3*Y-2: FOR I=1 TO 3
2030 FX=FX+DX:FY=FY+DY
2040 COLOR=0
2060 FOR K=0 TO 1: FOR L=0 TO 1: PLOT HX+K, HY+L: NEXT L,K: COLOR=8: FOR K=0 TO 1: FOR L=0 TO 1: PLOT FX+K, FY+L: NEXT L,K: HX=FX:HY=FY
2110 NEXT I
2115 X=X+DX:Y=Y+DY
2116 IF X=13 AND Y=WY THEN 6000
2120 GOTO 1500
2500 DX=-1:DY=0
2510 IF M(X+13*(Y-1)-1) MOD 10 THEN 4100
2520 GOTO 2020
3000 DX=0:DY=-1
3010 IF M(X+13*(Y-2))/10 THEN 4200
3020 GOTO 2020
3500 DX=0:DY=1
3510 IF M(X+13*(Y-1))/10 THEN 4306
3520 GOTO 2020


4000 GOSUB 5000
4010 COLOR=15
4020 VLIN 3*(Y-1),3*Y AT 3*X
4030 GOTO 1500
4100 GOSUB 5000
4110 COLOR=15
4120 VLIN 3*(Y-1),3*Y AT 3*(X-1)
4130 GOTO 1500
4200 GOSUB 5000
4210 COLOR=15
4220 HLIN 3*(X-1),3*X AT 3*(Y-1)
4230 GOTO 1500
4300 GOSUB 5000
4310 COLOR=15
4320 HLIN 3*(X-1),3*X AT 3*Y
4330 GOTO 1500
5000 S=S-1: FOR I=1 TO 20:A= PEEK (-16336)+ PEEK (-16336)+ PEEK (-16336)+ PEEK (-16336): NEXT I: RETURN
6000 PRINT "YOU WIN!"
6010 GOSUB 5000: GOSUB 5000: GOSUB: 5000
6020 PRINT "SCORE=";S+3
6030 END
7000 IF X>SX THEN 7005: IF Y>SY THEN 7050
7001 IF X<SX THEN 7100: IF Y<SY THEN 7150
7005 IF SX=13 THEN 7050: IF T(SX+13*(SY-1))>9 THEN 7010: IF M(SX+13*SY-1)) MOD 10 THEN 7050
7010 DX=1:DY=0
7020 COLOR=0
7022 RX=3*SX-2:RY=3*SY-2
7023 FOR I=1 TO 3:RX=RX+DX:RY=RY+DY
7024 COLOR=0
7025 FOR K=0 TO 1: FOR L=0 TO 1: PLOT QX+K,QY+L: NEXT L,K: COLOR=RD: FOR K=0 TO 1: FOR L=0 TO 1: PLOT RX+K,RY+L: NEXT L,K: QX=RX:QY=RY
7030 NEXT I
7035 SX=SX+DX:SY=SY+DY
7040 T(SX+13*(SY-1))=T(SX+13*(SY-1))+1
7045 RETURN
7050 IF SY=13 THEN 7100: IF T(SX+13*(SY-1))>9 THEN 7060: IF M(SX+13*(SY-1))/10 THEN 7100
7060 DX=0:DY=1: GOTO 7020
7100 IF SX=1 THEN 7150: IF T(SX+13*(SY-1))>9 THEN 7110: IF M(SX+13*(SY-1)-1) MOD 10 THEN 7150
7110 DX=-1:DY=0: GOTO 7020
7150 IF SY=1 THEN 7005: IF T(SX+13*(SY-1))>9 THEN 7160: IF M(SX+13*(SY-1)-13)/10 THEN 7005
7160 DX=0:DY=-1: GOTO 7020
8000 GOSUB 5000: GOSUB 5000: GOSUB: 5000
GOSUB 5000: PRINT "THE DRAGON GOT YOU!"
8999 END