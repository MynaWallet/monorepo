// SPDX-License-Identifier: GPL-3.0
/*
    Copyright 2021 0KIMS association.

    This file is generated with [snarkJS](https://github.com/iden3/snarkjs).

    snarkJS is a free software: you can redistribute it and/or modify it
    under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    snarkJS is distributed in the hope that it will be useful, but WITHOUT
    ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
    or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public
    License for more details.

    You should have received a copy of the GNU General Public License
    along with snarkJS. If not, see <https://www.gnu.org/licenses/>.
*/

pragma solidity >=0.7.0 <0.9.0;

contract MynaGovSigVerifier {
    // Scalar field size
    uint256 constant r = 21888242871839275222246405745257275088548364400416034343698204186575808495617;
    // Base field size
    uint256 constant q = 21888242871839275222246405745257275088696311157297823662689037894645226208583;

    // Verification Key data
    uint256 constant alphax = 20491192805390485299153009773594534940189261866228447918068658471970481763042;
    uint256 constant alphay = 9383485363053290200918347156157836566562967994039712273449902621266178545958;
    uint256 constant betax1 = 4252822878758300859123897981450591353533073413197771768651442665752259397132;
    uint256 constant betax2 = 6375614351688725206403948262868962793625744043794305715222011528459656738731;
    uint256 constant betay1 = 21847035105528745403288232691147584728191162732299865338377159692350059136679;
    uint256 constant betay2 = 10505242626370262277552901082094356697409835680220590971873171140371331206856;
    uint256 constant gammax1 = 11559732032986387107991004021392285783925812861821192530917403151452391805634;
    uint256 constant gammax2 = 10857046999023057135944570762232829481370756359578518086990519993285655852781;
    uint256 constant gammay1 = 4082367875863433681332203403145435568316851327593401208105741076214120093531;
    uint256 constant gammay2 = 8495653923123431417604973247489272438418190587263600148770280649306958101930;
    uint256 constant deltax1 = 13852857032342530930605263284505234428798439870600148209788340828000245353516;
    uint256 constant deltax2 = 8599542926507262441875995612307797602874511416313171215604391079363790013704;
    uint256 constant deltay1 = 4041894675951619254150017120139787509960291060047231188967935357150647832908;
    uint256 constant deltay2 = 2375183444257265201186845007594212213218041674327526084247182307321215296492;

    uint256 constant IC0x = 7441607095611835814370721145253227673217654661770423448604533840761042303372;
    uint256 constant IC0y = 15367515245831588277088609697681865685749545654849244998476560712492243993641;

    uint256 constant IC1x = 14171783463793022590939514084887512984176218159409336586823179825730764165305;
    uint256 constant IC1y = 2009768033625566762994716344124153559866622092237694483651002031474217574465;

    uint256 constant IC2x = 20820772984914871081814525329224973370866132006655672570513433539849322455167;
    uint256 constant IC2y = 4776914153773260730340910881786690787014330653242092719793354935108179402707;

    uint256 constant IC3x = 11746027375177829024109926138176902246935552120854329761164097829599746719619;
    uint256 constant IC3y = 6712012626780066646893774159147215666654407651995203459459413195921660120032;

    uint256 constant IC4x = 13786669858891948763012997047677075718819757271995299602424034984650216063886;
    uint256 constant IC4y = 415090816064777444778768334332555330639292530853087182421670208081088946170;

    uint256 constant IC5x = 1981376557028978208989076882796981496388055215143318125072896983537548920986;
    uint256 constant IC5y = 19528072067877882658072002029936306271323607429940069072230444256279394926592;

    uint256 constant IC6x = 7042906037013097364260195567569115377051451406411045893710625937555417153220;
    uint256 constant IC6y = 14995125626970316862803806598241999381085342140140796880713851847631101561717;

    uint256 constant IC7x = 9826859911059652025654963091188147218339739939658395412951214895666326952476;
    uint256 constant IC7y = 17599007867611229385435563670516765520227955135161665746098282002065364776485;

    uint256 constant IC8x = 17081148868780045850902266807195297089898954269827132700047926572542635727805;
    uint256 constant IC8y = 12628434754746498040276619954627910220259109784968300966380194257492360623564;

    uint256 constant IC9x = 20381351175661738298092414384231149431730848928897282211220419665377034985525;
    uint256 constant IC9y = 18282788276000372149290432256312095709643640732428811505235023591030987678825;

    uint256 constant IC10x = 11601420476805568222194449313479991561212335863938609254943490945507455397993;
    uint256 constant IC10y = 11789688312352428480616448100632713427170017175942454331877779197963277785261;

    uint256 constant IC11x = 10986258889065641317978123109278474486436062347349796735792365824357955449894;
    uint256 constant IC11y = 1209389751394551060854594452269425205849906682787373779091537957246990418685;

    uint256 constant IC12x = 1731915302763752670701492839144650216927487697287675493743351945486535451837;
    uint256 constant IC12y = 12021300103186005518947975814381615682457091496181941465530938309214433864036;

    uint256 constant IC13x = 8556698710959164832605015801401468675269469919690799389016627044775925087669;
    uint256 constant IC13y = 1877881866015269370109752669591460454985524555766135364758262562528077996029;

    uint256 constant IC14x = 4026106119283914487614065050680754737060425322599069769618807543738665124225;
    uint256 constant IC14y = 6132364646594623939773654448033269576977178664330827541205076667730325559844;

    uint256 constant IC15x = 9451576050881284268262004902867148813011320841279267478545923018595846950298;
    uint256 constant IC15y = 11911098848858571395269018315228079032958647913734600123005974837304677242237;

    uint256 constant IC16x = 18936753525571631196316777401783035544776792663249410899729075578737349244793;
    uint256 constant IC16y = 16421253311687590123631283126447418519383358698262285175084847381370121896070;

    uint256 constant IC17x = 829471031020235124210246388891434288950543934657382864224495967759707919650;
    uint256 constant IC17y = 15800481867573044422741295676713923213482539514026021981348634925509068095990;

    uint256 constant IC18x = 15904964386923502276544738807284717030611578200712406433933525348998840898006;
    uint256 constant IC18y = 18669073490480027534661326254375960460991698630991692112518939191530796586906;

    uint256 constant IC19x = 10951546514007317478599257072610570405291554115837153066320778290750539559941;
    uint256 constant IC19y = 3570725178592345896927782032623244403759332399957226875647792973275590040153;

    uint256 constant IC20x = 11759262635751583974711646640056895114502512562044838657827765866860860966231;
    uint256 constant IC20y = 15016478799268293255943509914829399149735136679000423380297347578563721313341;

    uint256 constant IC21x = 13192705612223929084809923890143122581968049558078135110824754062003310507021;
    uint256 constant IC21y = 10590171623658466128427540474171783637768476181465964244348519622972895049879;

    uint256 constant IC22x = 1471889866206805396159711869816953375941010852554847092008175390690209513389;
    uint256 constant IC22y = 12157160252460357423220648143238957805057236725013497714824057372883443634258;

    uint256 constant IC23x = 11163945311939619343575515927251640935272147552095836488337034567362329040879;
    uint256 constant IC23y = 7129695659593043332083811316125169152709006895742885452460555371702633874243;

    uint256 constant IC24x = 9429842543368542031036276607198531251474519645424410343270778521041319021134;
    uint256 constant IC24y = 8439108940450447316113134112530372786599610112749653820374766735934268104426;

    uint256 constant IC25x = 9085363710181100110583738507458654625362434313668892954616057290926201840330;
    uint256 constant IC25y = 621330159775331744079453153628607577285105772105196194435009203353723269292;

    uint256 constant IC26x = 5731757352166670511006852125258804425121757173262780137575800887808869506526;
    uint256 constant IC26y = 18587967088821329910377765105530555070480454658267826057554200620504613401080;

    uint256 constant IC27x = 7864471534308050600366895966229281403049344122062852081142751859620863110994;
    uint256 constant IC27y = 5687202315568827116064775048049954741778023088933845792928667243445903868137;

    uint256 constant IC28x = 4407093103856866296775854269791916540460388058767121872740019186989793716707;
    uint256 constant IC28y = 3898727957973103182578384635721981976011983256084850495977720923669524872590;

    uint256 constant IC29x = 14864547429686979215392658247655832045680417448268553962357006613904851971604;
    uint256 constant IC29y = 5873223892289456491843097689586109207701083104123587172681220915903463470371;

    uint256 constant IC30x = 9554753876320803199103086423340947243130277953560271654222346740132537428155;
    uint256 constant IC30y = 9380397378092346615153830329013606535784769000859847811844584814842117854424;

    uint256 constant IC31x = 4413421830831902467389924139256541973907744456192451245738954236795505725620;
    uint256 constant IC31y = 20737712816124400249795531839894902291569829659630228196380911627933250602414;

    uint256 constant IC32x = 3500166399558154574218611163773292482873678023823970310339328139871711793111;
    uint256 constant IC32y = 537839007554760582612352746389235030338180089680472011290279088670630665943;

    uint256 constant IC33x = 7067669891079075063873744411333439030216993560485066362624408255298368136287;
    uint256 constant IC33y = 13295205634239020336763618828260764917272263234136790003815074745380893940592;

    uint256 constant IC34x = 18781135059630372122141471301239767219692419819721924509276867617042525707173;
    uint256 constant IC34y = 1004861450402441175879283152588174085443129177780060875546831693757802061580;

    uint256 constant IC35x = 12536215438479813592975320666357951248111699441106426810072729061477064887112;
    uint256 constant IC35y = 21834545110610842187299508098992794175579683733367357672546714846875181703406;

    // Memory data
    uint16 constant pVk = 0;
    uint16 constant pPairing = 128;

    uint16 constant pLastMem = 896;

    function verifyProof(
        uint256[2] calldata _pA,
        uint256[2][2] calldata _pB,
        uint256[2] calldata _pC,
        uint256[35] calldata _pubSignals
    ) public view returns (bool) {
        assembly {
            function checkField(v) {
                if iszero(lt(v, q)) {
                    mstore(0, 0)
                    return(0, 0x20)
                }
            }

            // G1 function to multiply a G1 value(x,y) to value in an address
            function g1_mulAccC(pR, x, y, s) {
                let success
                let mIn := mload(0x40)
                mstore(mIn, x)
                mstore(add(mIn, 32), y)
                mstore(add(mIn, 64), s)

                success := staticcall(sub(gas(), 2000), 7, mIn, 96, mIn, 64)

                if iszero(success) {
                    mstore(0, 0)
                    return(0, 0x20)
                }

                mstore(add(mIn, 64), mload(pR))
                mstore(add(mIn, 96), mload(add(pR, 32)))

                success := staticcall(sub(gas(), 2000), 6, mIn, 128, pR, 64)

                if iszero(success) {
                    mstore(0, 0)
                    return(0, 0x20)
                }
            }

            function checkPairing(pA, pB, pC, pubSignals, pMem) -> isOk {
                let _pPairing := add(pMem, pPairing)
                let _pVk := add(pMem, pVk)

                mstore(_pVk, IC0x)
                mstore(add(_pVk, 32), IC0y)

                // Compute the linear combination vk_x

                g1_mulAccC(_pVk, IC1x, IC1y, calldataload(add(pubSignals, 0)))

                g1_mulAccC(_pVk, IC2x, IC2y, calldataload(add(pubSignals, 32)))

                g1_mulAccC(_pVk, IC3x, IC3y, calldataload(add(pubSignals, 64)))

                g1_mulAccC(_pVk, IC4x, IC4y, calldataload(add(pubSignals, 96)))

                g1_mulAccC(_pVk, IC5x, IC5y, calldataload(add(pubSignals, 128)))

                g1_mulAccC(_pVk, IC6x, IC6y, calldataload(add(pubSignals, 160)))

                g1_mulAccC(_pVk, IC7x, IC7y, calldataload(add(pubSignals, 192)))

                g1_mulAccC(_pVk, IC8x, IC8y, calldataload(add(pubSignals, 224)))

                g1_mulAccC(_pVk, IC9x, IC9y, calldataload(add(pubSignals, 256)))

                g1_mulAccC(_pVk, IC10x, IC10y, calldataload(add(pubSignals, 288)))

                g1_mulAccC(_pVk, IC11x, IC11y, calldataload(add(pubSignals, 320)))

                g1_mulAccC(_pVk, IC12x, IC12y, calldataload(add(pubSignals, 352)))

                g1_mulAccC(_pVk, IC13x, IC13y, calldataload(add(pubSignals, 384)))

                g1_mulAccC(_pVk, IC14x, IC14y, calldataload(add(pubSignals, 416)))

                g1_mulAccC(_pVk, IC15x, IC15y, calldataload(add(pubSignals, 448)))

                g1_mulAccC(_pVk, IC16x, IC16y, calldataload(add(pubSignals, 480)))

                g1_mulAccC(_pVk, IC17x, IC17y, calldataload(add(pubSignals, 512)))

                g1_mulAccC(_pVk, IC18x, IC18y, calldataload(add(pubSignals, 544)))

                g1_mulAccC(_pVk, IC19x, IC19y, calldataload(add(pubSignals, 576)))

                g1_mulAccC(_pVk, IC20x, IC20y, calldataload(add(pubSignals, 608)))

                g1_mulAccC(_pVk, IC21x, IC21y, calldataload(add(pubSignals, 640)))

                g1_mulAccC(_pVk, IC22x, IC22y, calldataload(add(pubSignals, 672)))

                g1_mulAccC(_pVk, IC23x, IC23y, calldataload(add(pubSignals, 704)))

                g1_mulAccC(_pVk, IC24x, IC24y, calldataload(add(pubSignals, 736)))

                g1_mulAccC(_pVk, IC25x, IC25y, calldataload(add(pubSignals, 768)))

                g1_mulAccC(_pVk, IC26x, IC26y, calldataload(add(pubSignals, 800)))

                g1_mulAccC(_pVk, IC27x, IC27y, calldataload(add(pubSignals, 832)))

                g1_mulAccC(_pVk, IC28x, IC28y, calldataload(add(pubSignals, 864)))

                g1_mulAccC(_pVk, IC29x, IC29y, calldataload(add(pubSignals, 896)))

                g1_mulAccC(_pVk, IC30x, IC30y, calldataload(add(pubSignals, 928)))

                g1_mulAccC(_pVk, IC31x, IC31y, calldataload(add(pubSignals, 960)))

                g1_mulAccC(_pVk, IC32x, IC32y, calldataload(add(pubSignals, 992)))

                g1_mulAccC(_pVk, IC33x, IC33y, calldataload(add(pubSignals, 1024)))

                g1_mulAccC(_pVk, IC34x, IC34y, calldataload(add(pubSignals, 1056)))

                g1_mulAccC(_pVk, IC35x, IC35y, calldataload(add(pubSignals, 1088)))

                // -A
                mstore(_pPairing, calldataload(pA))
                mstore(add(_pPairing, 32), mod(sub(q, calldataload(add(pA, 32))), q))

                // B
                mstore(add(_pPairing, 64), calldataload(pB))
                mstore(add(_pPairing, 96), calldataload(add(pB, 32)))
                mstore(add(_pPairing, 128), calldataload(add(pB, 64)))
                mstore(add(_pPairing, 160), calldataload(add(pB, 96)))

                // alpha1
                mstore(add(_pPairing, 192), alphax)
                mstore(add(_pPairing, 224), alphay)

                // beta2
                mstore(add(_pPairing, 256), betax1)
                mstore(add(_pPairing, 288), betax2)
                mstore(add(_pPairing, 320), betay1)
                mstore(add(_pPairing, 352), betay2)

                // vk_x
                mstore(add(_pPairing, 384), mload(add(pMem, pVk)))
                mstore(add(_pPairing, 416), mload(add(pMem, add(pVk, 32))))

                // gamma2
                mstore(add(_pPairing, 448), gammax1)
                mstore(add(_pPairing, 480), gammax2)
                mstore(add(_pPairing, 512), gammay1)
                mstore(add(_pPairing, 544), gammay2)

                // C
                mstore(add(_pPairing, 576), calldataload(pC))
                mstore(add(_pPairing, 608), calldataload(add(pC, 32)))

                // delta2
                mstore(add(_pPairing, 640), deltax1)
                mstore(add(_pPairing, 672), deltax2)
                mstore(add(_pPairing, 704), deltay1)
                mstore(add(_pPairing, 736), deltay2)

                let success := staticcall(sub(gas(), 2000), 8, _pPairing, 768, _pPairing, 0x20)

                isOk := and(success, mload(_pPairing))
            }

            let pMem := mload(0x40)
            mstore(0x40, add(pMem, pLastMem))

            // Validate that all evaluations ∈ F

            checkField(calldataload(add(_pubSignals, 0)))

            checkField(calldataload(add(_pubSignals, 32)))

            checkField(calldataload(add(_pubSignals, 64)))

            checkField(calldataload(add(_pubSignals, 96)))

            checkField(calldataload(add(_pubSignals, 128)))

            checkField(calldataload(add(_pubSignals, 160)))

            checkField(calldataload(add(_pubSignals, 192)))

            checkField(calldataload(add(_pubSignals, 224)))

            checkField(calldataload(add(_pubSignals, 256)))

            checkField(calldataload(add(_pubSignals, 288)))

            checkField(calldataload(add(_pubSignals, 320)))

            checkField(calldataload(add(_pubSignals, 352)))

            checkField(calldataload(add(_pubSignals, 384)))

            checkField(calldataload(add(_pubSignals, 416)))

            checkField(calldataload(add(_pubSignals, 448)))

            checkField(calldataload(add(_pubSignals, 480)))

            checkField(calldataload(add(_pubSignals, 512)))

            checkField(calldataload(add(_pubSignals, 544)))

            checkField(calldataload(add(_pubSignals, 576)))

            checkField(calldataload(add(_pubSignals, 608)))

            checkField(calldataload(add(_pubSignals, 640)))

            checkField(calldataload(add(_pubSignals, 672)))

            checkField(calldataload(add(_pubSignals, 704)))

            checkField(calldataload(add(_pubSignals, 736)))

            checkField(calldataload(add(_pubSignals, 768)))

            checkField(calldataload(add(_pubSignals, 800)))

            checkField(calldataload(add(_pubSignals, 832)))

            checkField(calldataload(add(_pubSignals, 864)))

            checkField(calldataload(add(_pubSignals, 896)))

            checkField(calldataload(add(_pubSignals, 928)))

            checkField(calldataload(add(_pubSignals, 960)))

            checkField(calldataload(add(_pubSignals, 992)))

            checkField(calldataload(add(_pubSignals, 1024)))

            checkField(calldataload(add(_pubSignals, 1056)))

            checkField(calldataload(add(_pubSignals, 1088)))

            checkField(calldataload(add(_pubSignals, 1120)))

            // Validate all evaluations
            let isValid := checkPairing(_pA, _pB, _pC, _pubSignals, pMem)

            mstore(0, isValid)
            return(0, 0x20)
        }
    }
}
