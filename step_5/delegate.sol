// SPDX-License-Identifier: MIT
pragma solidity <=0.8.28;

contract Called {
    uint public number;

    function increment() public {
        number++;
    }
}

contract Caller {
    uint public number;

    function callIncrement(address _calledAddress) public {
        _calledAddress.delegatecall(
            abi.encodeWithSignature("increment()")
        );
    }
}
