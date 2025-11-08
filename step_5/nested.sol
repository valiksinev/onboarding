// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract B {
    function fail() public pure  {
        require(false, "revert is called");
    }
}

contract A {
    function call_b(address addr) public  {
        (bool ok,) = addr.call(abi.encodeWithSignature("fail()"));
        require(ok == false);
    }

    function call_b_checked(address addr) public pure {
        B contract_b = B(addr);
        contract_b.fail();
    }
}
