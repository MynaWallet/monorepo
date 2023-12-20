// SPDX-License-Identifier: GPL-3.0
pragma solidity >=0.7.0 <0.9.0;

import "../interfaces/IMainMynaRegistrationVerifier.sol";

contract MainMynaRegistrationVerifier is IMainMynaRegistrationVerifier {
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
    uint256 constant deltax1 = 14381092624764922233793960000343825773920879806154828935092756837651807259719;
    uint256 constant deltax2 = 21388153354377210547957709733590429473441346725163022831661910526447408606507;
    uint256 constant deltay1 = 18353784087927229850044845576817460320943246523623986673733369983284862336388;
    uint256 constant deltay2 = 13708114993451334824680746074619414012698403668375359098090318392273577847590;

    uint256 constant IC0x = 13280572492875496423415001511821187782820375039323537748101643560488357283640;
    uint256 constant IC0y = 5560551206796373571252117640309183466286768235831205401362422186003447967838;

    uint256 constant IC1x = 9987290775146690112011425830250816953380515686227675782174905165895344398989;
    uint256 constant IC1y = 9267436533408351648953313987005894263710505351386168088017238122266909420758;

    uint256 constant IC2x = 1695208939396229854526339549469220183848848785395029177624428937281627311039;
    uint256 constant IC2y = 14266565435905680004835335454442165283658049703057343113494152095026611478843;

    uint256 constant IC3x = 1953150851220832081269065747447085703241463324759123810754120687678396341560;
    uint256 constant IC3y = 2826036882401052759586057935677491847967552918411911980158389689296326333315;

    uint256 constant IC4x = 14157223232428182905711013847667379676158838976261618999798969528977615801685;
    uint256 constant IC4y = 3789043429743997121461218000588952160370331848960776009906157152452747891347;

    uint256 constant IC5x = 17029134442212063891370375467425128889097555526396837112057361708386775367476;
    uint256 constant IC5y = 15398323851898270414292017528548120673597991205620203390221289415435937847212;

    uint256 constant IC6x = 4923840918071156437887605910727853990140299517524551622866131149730183147214;
    uint256 constant IC6y = 5867210147994303260655010103164750968361252409679424191909955834047956031581;

    uint256 constant IC7x = 15285771374529382722006734662260613653634051266840817567519982063172360584859;
    uint256 constant IC7y = 15907471087074356223248155159399218505265113017703321915718447622547874049998;

    uint256 constant IC8x = 20149106003034670690359295201461178590334471503544128598167180496152876091692;
    uint256 constant IC8y = 2832847492943069332491195566156008075455949582923643716841053031272948628458;

    uint256 constant IC9x = 7081069505591330111437435322741935807005435376917615196109398594946447070783;
    uint256 constant IC9y = 6082840881156239221070747634017381103275575151566628478433188703398206657105;

    uint256 constant IC10x = 9352840052435641593835572349525033876465966709733832411080515348812478757561;
    uint256 constant IC10y = 2950368612480035608628354869860112704138369943829117714858774169528525175404;

    uint256 constant IC11x = 5194244107666611688007642950022554597675171603981893415780439323844367124488;
    uint256 constant IC11y = 1768376628044308905648207512347557364664613781151432696961572454353123878905;

    uint256 constant IC12x = 4066553114536992424062573983593406069650213648832358129745977551430004443767;
    uint256 constant IC12y = 16638738789996800362412381903361905474421630949535995698087193947408976530842;

    uint256 constant IC13x = 1811275305974524236322086955381085669284522395265195212724375346800772496002;
    uint256 constant IC13y = 2206703231090143454431759588022675559543640411399890866112701394211535194124;

    uint256 constant IC14x = 11346739230244707700288362704898627714343896456268110620426819631077143275147;
    uint256 constant IC14y = 6137848857574448162662521562437710142277728282180897723774622863306241515871;

    uint256 constant IC15x = 6117165523828788080294808829462102402249859728283677209770919290758852558472;
    uint256 constant IC15y = 6767597811169130705044732411407011833246282329721624370031604494180371956249;

    uint256 constant IC16x = 19093915901698731859072220997281937944776627590587237050332307175887697857998;
    uint256 constant IC16y = 20287789735204535172535371617359537842629909331807219956008954290999428444807;

    uint256 constant IC17x = 20027690484266360780839643964873628890870700916994375455272050452105036634408;
    uint256 constant IC17y = 8724401561878684588257450246290358988643882279397869513354181699840565620134;

    uint256 constant IC18x = 17547994128468455146993929160677167957212023610744502176250921512867034845968;
    uint256 constant IC18y = 5806117339386228330786534083563379765587442611509884994396413629835278412856;

    uint256 constant IC19x = 15975464175274514534136124443797011903090698958887713027109894219504424548424;
    uint256 constant IC19y = 19987789922826767672199362615535071700941688641876456983060624210812848190997;

    // Memory data
    uint16 constant pVk = 0;
    uint16 constant pPairing = 128;

    uint16 constant pLastMem = 896;

    function verifyProof(
        uint256[2] calldata _pA,
        uint256[2][2] calldata _pB,
        uint256[2] calldata _pC,
        uint256[19] calldata _pubSignals
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

            // Validate all evaluations
            let isValid := checkPairing(_pA, _pB, _pC, _pubSignals, pMem)

            mstore(0, isValid)
            return(0, 0x20)
        }
    }
}
