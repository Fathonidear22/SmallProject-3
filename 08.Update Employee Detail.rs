<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>08.Update Employee Detail</name>
   <tag></tag>
   <elementGuidId>9d869d5b-eb1e-4618-9c8a-3ed434d4f306</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;application/x-www-form-urlencoded&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;id&quot;,
      &quot;value&quot;: &quot;Number&quot;
    },
    {
      &quot;name&quot;: &quot;firstName&quot;,
      &quot;value&quot;: &quot;String&quot;
    },
    {
      &quot;name&quot;: &quot;middleName&quot;,
      &quot;value&quot;: &quot;String&quot;
    },
    {
      &quot;name&quot;: &quot;lastName&quot;,
      &quot;value&quot;: &quot;String&quot;
    },
    {
      &quot;name&quot;: &quot;code&quot;,
      &quot;value&quot;: &quot;String&quot;
    },
    {
      &quot;name&quot;: &quot;dob&quot;,
      &quot;value&quot;: &quot;String&quot;
    },
    {
      &quot;name&quot;: &quot;licenseNumber&quot;,
      &quot;value&quot;: &quot;String&quot;
    },
    {
      &quot;name&quot;: &quot;licenseNumberExpDate&quot;,
      &quot;value&quot;: &quot;String&quot;
    },
    {
      &quot;name&quot;: &quot;maritalStatus&quot;,
      &quot;value&quot;: &quot;String&quot;
    },
    {
      &quot;name&quot;: &quot;gander&quot;,
      &quot;value&quot;: &quot;String&quot;
    },
    {
      &quot;name&quot;: &quot;otherId&quot;,
      &quot;value&quot;: &quot;String&quot;
    },
    {
      &quot;name&quot;: &quot;nationality&quot;,
      &quot;value&quot;: &quot;String&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>x-www-form-urlencoded</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${GlobalVariable.TestToken}</value>
   </httpHeaderProperties>
   <katalonVersion>8.0.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://qa.cilsy.id/symfony/web/index.php/api/v1/employee/3</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
