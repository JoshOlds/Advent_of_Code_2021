pub static PUZZLE_INPUT: &str = "103
112
111
133
132
128
136
138
133
136
137
140
144
156
155
172
171
172
185
203
206
209
208
212
209
210
211
214
245
247
249
246
274
276
294
292
297
291
304
311
325
336
337
344
350
357
359
356
379
381
382
384
379
375
376
377
376
369
373
338
339
365
373
385
394
395
396
399
400
406
439
440
438
443
433
435
432
451
443
427
422
428
429
436
435
437
433
447
468
469
478
475
494
493
495
499
506
492
494
493
494
498
499
521
546
547
549
550
551
550
547
570
575
578
595
602
594
582
592
597
592
571
562
556
558
560
569
570
585
558
574
567
576
577
591
592
600
603
601
594
601
592
614
615
617
623
646
647
650
652
683
686
687
688
689
690
694
705
706
723
719
723
728
739
738
751
769
770
794
797
793
794
819
826
829
832
844
845
847
839
847
861
859
860
861
857
859
860
858
869
857
835
832
860
868
872
873
871
850
866
865
891
892
896
914
916
918
922
921
922
930
931
959
957
958
971
969
970
969
975
974
968
981
994
989
995
998
1006
1009
1015
1017
1018
1028
1029
1008
1013
1035
1056
1062
1064
1065
1066
1065
1067
1068
1073
1075
1078
1083
1063
1067
1069
1071
1074
1077
1076
1072
1098
1099
1084
1085
1057
1061
1081
1090
1094
1095
1096
1090
1094
1095
1102
1133
1132
1144
1147
1137
1140
1146
1147
1148
1149
1150
1156
1175
1184
1187
1191
1192
1201
1237
1239
1240
1241
1236
1237
1263
1268
1271
1265
1283
1288
1301
1309
1320
1321
1314
1310
1309
1322
1323
1324
1318
1317
1319
1318
1312
1328
1332
1331
1332
1349
1345
1363
1356
1357
1356
1360
1359
1368
1379
1395
1389
1390
1397
1388
1389
1390
1392
1414
1420
1421
1422
1427
1437
1438
1432
1430
1459
1458
1466
1467
1469
1470
1480
1478
1479
1478
1484
1485
1491
1494
1496
1504
1514
1536
1568
1592
1599
1602
1603
1609
1611
1612
1613
1627
1626
1632
1640
1651
1657
1658
1665
1682
1683
1691
1692
1695
1698
1704
1703
1710
1720
1721
1729
1727
1728
1729
1730
1734
1735
1737
1742
1748
1749
1758
1760
1761
1767
1768
1770
1771
1757
1770
1807
1821
1825
1828
1827
1842
1854
1855
1860
1859
1851
1849
1851
1855
1858
1888
1887
1899
1935
1938
1921
1918
1920
1924
1923
1925
1928
1925
1930
1931
1942
1943
1952
1959
1951
1952
1951
1946
1956
1965
1982
1978
1980
1982
1977
1979
1978
1986
1988
2000
1979
1986
1988
1955
1946
1951
1965
1967
1968
1967
1973
1976
1977
1976
1975
1992
2004
2009
2013
2016
2015
2018
2016
2020
2004
2003
2031
2020
2022
2055
2059
2056
2058
2059
2062
2063
2069
2098
2122
2112
2111
2119
2117
2118
2145
2146
2145
2148
2149
2182
2188
2218
2226
2233
2235
2242
2243
2242
2261
2274
2280
2273
2283
2311
2318
2322
2324
2314
2323
2324
2328
2335
2337
2334
2335
2346
2342
2356
2360
2361
2392
2396
2412
2419
2423
2426
2442
2441
2412
2414
2422
2428
2441
2457
2458
2459
2461
2462
2467
2468
2469
2479
2483
2487
2493
2499
2503
2516
2510
2520
2521
2518
2504
2512
2513
2529
2530
2533
2516
2514
2509
2518
2527
2530
2534
2530
2533
2542
2557
2558
2560
2555
2557
2569
2570
2580
2591
2577
2602
2603
2608
2609
2596
2594
2605
2603
2607
2611
2614
2616
2617
2618
2617
2624
2621
2626
2630
2631
2614
2619
2622
2640
2641
2654
2658
2661
2666
2686
2695
2722
2721
2722
2723
2698
2705
2706
2710
2730
2732
2738
2745
2752
2753
2750
2748
2749
2747
2749
2754
2763
2749
2747
2743
2735
2737
2729
2725
2700
2711
2721
2698
2704
2705
2706
2707
2725
2728
2733
2734
2741
2742
2753
2755
2756
2757
2756
2762
2765
2766
2768
2769
2790
2796
2833
2820
2821
2829
2842
2839
2841
2846
2841
2844
2839
2841
2821
2822
2841
2852
2858
2870
2862
2857
2879
2895
2876
2867
2877
2878
2880
2881
2891
2888
2887
2889
2891
2900
2902
2919
2920
2922
2941
2948
2946
2947
2942
2943
2940
2941
2943
2947
2938
2944
2930
2929
2928
2935
2938
2941
2946
2951
2955
2957
2958
2961
2965
2986
2981
2990
2991
2997
2988
2992
2987
2983
2992
2986
3013
3027
3015
3012
3020
3028
3030
3033
3032
3035
3038
3040
3041
3057
3056
3061
3037
3032
3038
3040
3059
3057
3058
3059
3068
3066
3065
3072
3053
3059
3066
3067
3084
3086
3090
3088
3110
3108
3126
3143
3150
3168
3162
3163
3166
3160
3156
3160
3165
3175
3174
3179
3207
3209
3211
3228
3235
3237
3239
3250
3251
3253
3256
3257
3258
3267
3272
3258
3275
3279
3286
3290
3295
3292
3293
3296
3305
3319
3321
3327
3332
3333
3350
3351
3356
3355
3357
3346
3355
3356
3354
3355
3351
3352
3353
3358
3359
3380
3382
3395
3394
3395
3398
3402
3404
3405
3398
3415
3427
3437
3431
3430
3435
3431
3440
3442
3425
3438
3449
3462
3467
3476
3477
3481
3484
3481
3483
3484
3483
3478
3483
3485
3496
3490
3497
3499
3500
3515
3508
3495
3501
3502
3512
3511
3513
3512
3525
3553
3552
3555
3556
3557
3556
3560
3561
3551
3553
3579
3587
3588
3600
3588
3584
3588
3589
3590
3597
3590
3589
3590
3593
3592
3596
3598
3591
3597
3620
3621
3612
3617
3618
3623
3628
3629
3635
3650
3651
3652
3659
3642
3647
3643
3644
3653
3654
3650
3656
3657
3663
3664
3671
3672
3675
3673
3664
3647
3669
3670
3675
3673
3675
3697
3717
3718
3717
3718
3728
3741
3742
3748
3766
3772
3736
3740
3762
3783
3788
3789
3792
3801
3804
3772
3771
3768
3746
3745
3783
3806
3795
3810
3806
3802
3801
3815
3805
3804
3823
3827
3848
3861
3843
3842
3863
3875
3882
3879
3888
3882
3883
3879
3886
3888
3874
3872
3876
3874
3865
3873
3874
3870
3879
3876
3880
3881
3880
3878
3879
3902
3879
3881
3866
3878
3874
3871
3876
3858
3851
3855
3846
3866
3863
3857
3858
3850
3851
3852
3854
3878
3879
3881
3882
3886
3906
3907
3893
3921
3923
3935
3941
3920
3914
3954
3964
3965
3985
3990
3988
3986
3985
4001
3999
4004
4006
4016
4009
4037
4035
4036
4037
4038
4041
4046
4044
4032
4037
4039
4042
4031
4017
4020
4034
4035
4037
4045
4052
4058
4052
4053
4054
4055
4063
4064
4061
4057
4058
4067
4068
4071
4073
4075
4079
4084
4083
4050
4056
4057
4049
4069
4070
4071
4072
4080
4071
4070
4087
4095
4123
4117
4130
4146
4153
4158
4161
4164
4177
4191
4192
4165
4180
4184
4190
4193
4194
4172
4178
4161
4153
4152
4151
4169
4176
4172
4170
4172
4175
4179
4197
4213
4212
4213
4214
4213
4215
4216
4221
4212
4211
4215
4218
4217
4214
4227
4235
4236
4255
4256
4257
4258
4259
4267
4268
4263
4264
4265
4267
4273
4290
4313
4321
4324
4335
4333
4335
4341
4342
4344
4354
4356
4365
4368
4370
4371
4377
4380
4381
4358
4366
4369
4370
4371
4354
4355
4353
4362
4364
4367
4369
4374
4378
4379
4372
4381
4386
4389
4401
4402
4393
4395
4397
4398
4393
4421
4418
4424
4429
4433
4425
4433
4406
4407
4413
4414
4415
4418
4425
4423
4420
4422
4423
4434
4436
4449
4452
4428
4431
4440
4448
4450
4451
4452
4453
4447
4448
4457
4458
4446
4442
4445
4453
4455
4456
4457
4460
4462
4472
4498
4524
4525
4527
4528
4529
4530
4531
4528
4526
4553
4560
4569
4588
4589
4596
4595
4596
4594
4619
4610
4627
4626
4622
4637
4641
4642
4647
4648
4659
4660
4661
4663
4675
4681
4687
4688
4684
4676
4681
4695
4694
4691
4693
4692
4693
4694
4707
4711
4712
4727
4734
4717
4715
4719
4725
4731
4725
4723
4738
4739
4738
4748
4758
4762
4765
4764
4774
4792
4797
4798
4799
4802
4800
4816
4821
4831
4830
4836
4838
4822
4817
4820
4843
4844
4842
4857
4867
4864
4865
4871
4872
4873
4872
4873
4877
4879
4880
4895
4900
4896
4902
4901
4911
4906
4889
4893
4894
4899
4902
4896
4905
4903
4874
4891
4899
4913
4915
4927
4939
4941
4934
4949
4966
4962
4969
4970
4975
4973
4975
4977
4978
4972
4973
4976
4989
4994
4988
4992
4990
4996
5002
5003
5004
4997
5000
5002
5004
5001
5007
5008
5010
5011
5010
4998
5015
5028
5031
5051
5056
5057
5060
5092
5091
5092
5105
5094
5104
5110
5120
5121
5119
5124
5119
5114
5118
5119
5120
5123
5125
5127
5130
5133
5125
5126
5116
5117
5118
5104
5078
5075
5083
5085
5087
5095
5111
5112
5111
5117
5125
5136
5137
5138
5133
5153
5152
5164
5162
5175
5179
5163
5186
5192
5186
5178
5185
5184
5187
5193
5194
5201
5215
5224
5227
5234
5235
5236
5216
5202
5203
5206
5216
5219
5215
5225
5229
5233
5238
5242
5229
5230
5229
5228
5235
5237
5258
5259
5260
5262
5265
5266
5261
5265
5259
5258
5259
5252
5250
5236
5268
5269
5277
5278
5283
5284
5286
5293
5295
5296
5298
5299
5301
5303
5306
5313
5318
5319
5317
5323
5316
5344
5349
5355
5373
5383
5384
5386
5388
5389
5390
5389
5387
5395
5397
5393
5397
5379
5383
5412
5411
5412
5416
5414
5412
5395
5396
5397
5395
5396
5362
5361
5363
5364
5385
5388
5393
5382
5396
5399
5397
5402
5404
5407
5422
5424
5425
5437
5440
5438
5439
5437
5460
5454
5450
5445
5428
5429
5436
5441
5444
5442
5435
5446
5448
5456
5457
5454
5465
5466
5469
5471
5470
5490
5493
5514
5516
5517
5532
5525
5516
5517
5523
5520
5523
5538
5539
5538
5540
5551
5543
5550
5526
5527
5524
5538
5542
5545
5544
5548
5547
5559
5560
5581
5598
5599
5576
5585
5589
5590
5591
5575
5578
5587
5603
5604
5607
5609
5612
5613
5608
5609
5614
5620
5621
5633
5643
5647
5649
5652
5651
5660
5667
5677
5678
5695
5696
5697
5698
5699
5696
5687
5679
5695
5696
5705
5694
5679
5677
5706
5709
5711
5714
5715
5719
5720
5726
5727
5747
5750
5755
5757
5762
5758
5759
5782
5785
5783
5789
5792
5787
5788
5808
5811
5804
5795
5808
5809
5789
5777
5810
5788
5791
5793
5798
5790
5796
5800
5813
5819
5831
5830
5828
5811
5816
5826
5839
5841
5845
5852
5849
5850
5852
5867
5875
5899
5901
5902
5924
5931
5932
5931
5930
5931
5926
5942
5967
5958
5968
5996
5998
6003
5997
6016
6018
6036
6044
6058
6060
6075
6101
6086
6087
6097
6109
6110
6111
6110
6129
6119
6142
6149
6162
6145
6139
6133
6140
6124
6130
6141
6144
6151
6155
6159
6156
6161
6167
6171
6158
6164
6176
6177
6184
6160
6161
6166
6167
6176
6174
6146
6162
6193
6192
6194
6193
6202
6205
6204
6212
6226
6230
6240
6249
6241
6227
6225
6235
6237
6238
6240
6241
6243
6252
6254
6258
6266
6275
6295
6306
6324
6318
6319
6320
6309
6311
6317
6304
6308
6319
6321
6323
6318
6295
6299
6300
6314
6325
6308
6307
6310
6321
6333
6334
6327
6330
6313
6328
6330
6348
6343
6346
6359
6367
6373
6386
6382
6385
6387
6397
6398
6397
6400
6402
6405
6406
6408
6424
6449
6448
6464
6465
6470
6472
6494
6492
6503
6513
6525
6526
6527
6528
6536
6546
6545
6552
6557
6558
6568
6556
6557
6558
6557
6556
6555
6558
6557
6560
6566
6565
6570
6574
6568
6572
6589
6590
6589
6575
6588
6596
6597
6596
6561
6562
6564
6569
6578
6581
6569
6559
6571
6576
6581
6565
6567
6569
6572
6567
6568
6569
6575
6584
6585
6588
6590
6586
6593
6595
6599
6613
6647
6677
6673
6677
6676
6677
6698
6699
6698
6702
6712
6736
6734
6737
6741
6740";