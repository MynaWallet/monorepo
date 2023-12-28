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

contract Groth16Verifier {
    // Scalar field size
    uint256 constant r    = 21888242871839275222246405745257275088548364400416034343698204186575808495617;
    // Base field size
    uint256 constant q   = 21888242871839275222246405745257275088696311157297823662689037894645226208583;

    // Verification Key data
    uint256 constant alphax  = 20491192805390485299153009773594534940189261866228447918068658471970481763042;
    uint256 constant alphay  = 9383485363053290200918347156157836566562967994039712273449902621266178545958;
    uint256 constant betax1  = 4252822878758300859123897981450591353533073413197771768651442665752259397132;
    uint256 constant betax2  = 6375614351688725206403948262868962793625744043794305715222011528459656738731;
    uint256 constant betay1  = 21847035105528745403288232691147584728191162732299865338377159692350059136679;
    uint256 constant betay2  = 10505242626370262277552901082094356697409835680220590971873171140371331206856;
    uint256 constant gammax1 = 11559732032986387107991004021392285783925812861821192530917403151452391805634;
    uint256 constant gammax2 = 10857046999023057135944570762232829481370756359578518086990519993285655852781;
    uint256 constant gammay1 = 4082367875863433681332203403145435568316851327593401208105741076214120093531;
    uint256 constant gammay2 = 8495653923123431417604973247489272438418190587263600148770280649306958101930;
    uint256 constant deltax1 = 14505449069819768088354098292210814719179743826807109117503739337247919373744;
    uint256 constant deltax2 = 5714560068618788921367643424780126238521548012601045804519599482235803410713;
    uint256 constant deltay1 = 17875334920326811149671998415760814056463722658105284679824246501596093609927;
    uint256 constant deltay2 = 16769118834165472360695059231484850962302942338612282667079818561052804311254;

    
    uint256 constant IC0x = 13070892703966083814601157153802142960921339786240361966225106642991476103784;
    uint256 constant IC0y = 12383598677006914645900468679242076169528873538543785932018427995330480900945;
    
    uint256 constant IC1x = 12132419871480382009205960828558225698396432314361387801515929515303853257463;
    uint256 constant IC1y = 6924826189454965194872635380763628800616132812069103604980154830154195631833;
    
    uint256 constant IC2x = 6211769517872677301844817058181954849731195502894972555425088347728959123416;
    uint256 constant IC2y = 1810891898428559034979327787763790240530638774504867250442653843438550963738;
    
    uint256 constant IC3x = 8205165125723400373457677546980449588823678798171737096314486716632618194584;
    uint256 constant IC3y = 9455392612278141543219429436216989510245862903509882528849362381363202653331;
    
    uint256 constant IC4x = 11616611026888726640845329522312064955000712047968469374116945817129096425226;
    uint256 constant IC4y = 10711046360587420644065509253613491141699450290761488270937037230378213882650;
    
    uint256 constant IC5x = 16639717730589119980398982196493429037268424045257077459903160415243944081599;
    uint256 constant IC5y = 19445994573392013319475551697494023293524170814413125105047485453289288822991;
    
    uint256 constant IC6x = 2064597773409920317495965404075443707871764783959876790195900990800273313865;
    uint256 constant IC6y = 3036864082302582355267628860618478359444027777812635661247297223850929641676;
    
    uint256 constant IC7x = 4866207176957400568621478019519448013939815616695355441325730461163801148177;
    uint256 constant IC7y = 6910575706110081228226218678024950859742834212446339983086207997972907514449;
    
    uint256 constant IC8x = 21446361404560497392911690435295485091617030103810292504703811221310641204830;
    uint256 constant IC8y = 456160796996374151872478554534567911302043979198683807774096485051899469161;
    
    uint256 constant IC9x = 1163299285339267275598560925265582601134215896846031269361638689671545132977;
    uint256 constant IC9y = 19029106466370141364815940671855252860180587846090442848540484690874711100781;
    
    uint256 constant IC10x = 15671491004895292796238120141348234868112557067925999511626122273464941110664;
    uint256 constant IC10y = 18360021285189868398603857317087391523421770662068889188770517210257435247953;
    
    uint256 constant IC11x = 12611261410536848002823841503944256644611214492966958559156072184151481854862;
    uint256 constant IC11y = 1107043838739580927076095570391578730931244449439800217082346967945945848668;
    
    uint256 constant IC12x = 16429892902614967052239499090043719986657113534031680136375040104249006732756;
    uint256 constant IC12y = 6753306592546927089751255934276693844919670908791026700670621337483509840386;
    
    uint256 constant IC13x = 2202331561041235548561922406366422949792769139416496133058557061490022053281;
    uint256 constant IC13y = 10675436457408540483680656430033478420226658097160591301510904200205094703067;
    
    uint256 constant IC14x = 3665281389626974309882520101968579762923985451272272789988850382480690010933;
    uint256 constant IC14y = 21429049600503804749981945676427097329226182996004313532464312761200407649269;
    
    uint256 constant IC15x = 7541394858640697773118559048121772616448168860935474473326486433518128382560;
    uint256 constant IC15y = 4439932671222727572106809731425107309679102309960909070644363160060698102690;
    
    uint256 constant IC16x = 6663315638061457608766070048815872806632733215519185801701737476881786865963;
    uint256 constant IC16y = 2575780526383129716453202831099960028061232132443804320130415695936216259717;
    
    uint256 constant IC17x = 5609340015751550746619547601583043794825600988897968650551492981253985452646;
    uint256 constant IC17y = 715881069482396028876830914507321234931853782527061201891946684364838519021;
    
    uint256 constant IC18x = 324910991914514071760446532933314290526903954736247072838520096490729259809;
    uint256 constant IC18y = 8964199846596706990352298074924885555397594152522670324974645096804858180880;
    
    uint256 constant IC19x = 6930616049766129778804474921553720761266240278438687351713518884243271796936;
    uint256 constant IC19y = 5547327027686619462397584257921548420597071414797842730485331873477711285107;
    
    uint256 constant IC20x = 14635626019883479400252058740307362813708490182783849552442555527924972109849;
    uint256 constant IC20y = 18631885748748039005144607227731194780044628597017466330977905295090063758400;
    
    uint256 constant IC21x = 5169220109502469262398424749357429027161733990353521487854737149435996450811;
    uint256 constant IC21y = 7538490577352119752876168740505474656102782464036637963451275613550536903496;
    
    uint256 constant IC22x = 3276727462056427860508038642094934482944269517106309999402748342157820184365;
    uint256 constant IC22y = 150139866240443064820591650230062579380246890185591622222900648768489763265;
    
 
    // Memory data
    uint16 constant pVk = 0;
    uint16 constant pPairing = 128;

    uint16 constant pLastMem = 896;

    function verifyProof(uint[2] calldata _pA, uint[2][2] calldata _pB, uint[2] calldata _pC, uint[22] calldata _pubSignals) public view returns (bool) {
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
            

            // Validate all evaluations
            let isValid := checkPairing(_pA, _pB, _pC, _pubSignals, pMem)

            mstore(0, isValid)
             return(0, 0x20)
         }
     }
 }
