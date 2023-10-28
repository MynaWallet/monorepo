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

pragma solidity ^0.8.0;

contract MynaWalletVerifier {
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
    uint256 constant deltax1 = 20779241005383029176559982572538997002275373402932287969530801805179153419071;
    uint256 constant deltax2 = 12122910600117845814099682164636970742594116711351901646754319013208628143658;
    uint256 constant deltay1 = 1983017526031751716629415239560413638157889269186229814725555780394379698422;
    uint256 constant deltay2 = 18369035429828016927878365630266075205183849545611214333046850724751493873104;

    uint256 constant IC0x = 18224752695618958914183367431492413747486974162490887344569385249646648416277;
    uint256 constant IC0y = 5016344216128181760067345326058572609715084361926468524224835931238782977838;

    uint256 constant IC1x = 6059317173897392605134714272633960475425379859491719431954785157721471599975;
    uint256 constant IC1y = 2060850966544339708095907308198167470739932181575372182643862053135753693070;

    uint256 constant IC2x = 18282813080204901167679867679055977094581113656384639829228526185286738932121;
    uint256 constant IC2y = 10791222681768874464114320293845753513082517128066278337597342217673067887365;

    uint256 constant IC3x = 3013151660037470271261107953729889904243682739886788542650356249650495374695;
    uint256 constant IC3y = 526857636788171076328572166131060146551999748990135337494982404579591968202;

    uint256 constant IC4x = 9443343640189630191076581270932201607362979847608323433315185537412933655542;
    uint256 constant IC4y = 8833799068476557262014945742067376333751017957424743828411073191911663267833;

    uint256 constant IC5x = 14850536243611733795329936220909668491869310322694945195707734835291630143901;
    uint256 constant IC5y = 1709647218464574372136717294595056848809884606498797320641157459787975626124;

    uint256 constant IC6x = 16139871469219678297290800611437802898377767292452766911289056860544635929732;
    uint256 constant IC6y = 13742709323810489951673435908982039480952668516876374293822419728241746672378;

    uint256 constant IC7x = 7047194401632673208299706261038930233478876585334606698029113390955767008057;
    uint256 constant IC7y = 2541524861352502427583078797306596217464070430869523008803895772957841859766;

    uint256 constant IC8x = 15024864838533969060292222218177093343624448060611163032058927792131050122381;
    uint256 constant IC8y = 6400643622287390603283205693136418038812333699691586916132850518466981325149;

    uint256 constant IC9x = 11853177513692109942752215732529669062315558047822934848729983128330802586461;
    uint256 constant IC9y = 2865985529696148720671386057492112946056096338982156981128361211643313620938;

    uint256 constant IC10x = 6378304716938481122847809065105472541204942089078050205109063349322432984900;
    uint256 constant IC10y = 10931070413461982775314879186861812799573513886424529269978136175538736954445;

    uint256 constant IC11x = 18012305536023997400083177927808294621277377888220994027701273165228762962300;
    uint256 constant IC11y = 14648366948752260163181799462548295220986153564740147159901914053297573442566;

    uint256 constant IC12x = 1201812395370722002476994910015306558919219294595463506106165551267196942971;
    uint256 constant IC12y = 952131672792361038846655851210805944181021035432872835867623052808976304353;

    uint256 constant IC13x = 10582579333531572545087119271897239919594477767699004565820101514496536511184;
    uint256 constant IC13y = 19249987997948366858970940820873923171807650846179830374964822212546767174618;

    uint256 constant IC14x = 20284536952008736413258697099333070095955991514738907644973951384838709245759;
    uint256 constant IC14y = 14863512760938475739985573599954307681252495367085526287712141169048988048500;

    uint256 constant IC15x = 19807795794980605032298521455897784312255516282779810703768156274102023990720;
    uint256 constant IC15y = 21226695824475030864847110517562195719616439458753040940716611489699483326190;

    uint256 constant IC16x = 15723023662004255097460609831104739899704269755355512753139002644336310237230;
    uint256 constant IC16y = 19159349102650659082795576736753586613277013686551830630176531240472940595114;

    uint256 constant IC17x = 12733065217464606362656371495352097496217972591047292921186025504061093744565;
    uint256 constant IC17y = 17783329202487447264203353164169427467330088667128155181177549726206982106955;

    uint256 constant IC18x = 20465699600036170699315146498884651953469194901404967495835522224729354548540;
    uint256 constant IC18y = 12453736462859193104688935483054115598306455522122878170090804138242772578214;

    uint256 constant IC19x = 7369434679629416780477364422707709158434062294946282907089537787371678650562;
    uint256 constant IC19y = 13823650545816654242698070478089010465548704163597744735352155963406189974511;

    uint256 constant IC20x = 10728298454663256248021813977301755613432314795052466216248756107581828055055;
    uint256 constant IC20y = 18868896210970637062188438590208603685415511009600333475452626578829430734803;

    uint256 constant IC21x = 10420629878209129172862932593558477770866518008031585898847510768055914947279;
    uint256 constant IC21y = 16430970863328720347967054033196342017079039594334615709303455911037894559207;

    // Memory data
    uint16 constant pVk = 0;
    uint16 constant pPairing = 128;

    uint16 constant pLastMem = 896;

    function verifyProof(
        uint256[2] calldata _pA,
        uint256[2][2] calldata _pB,
        uint256[2] calldata _pC,
        uint256[21] calldata _pubSignals
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

            // Validate all evaluations
            let isValid := checkPairing(_pA, _pB, _pC, _pubSignals, pMem)

            mstore(0, isValid)
            return(0, 0x20)
        }
    }
}
